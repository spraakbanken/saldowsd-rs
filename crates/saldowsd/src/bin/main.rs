use std::io;

use asp_server::Connection;
use clap::Parser;
use log::LevelFilter;
use saldowsd::{cli::flags, config::Config, from_json};
use wsd_application::{
    SourceFormat, TabFormat, VectorWSD, VectorWSDConfig,
    wsd_application::{DisambiguateOptions, disambiguate_sentences},
};

fn main() -> eyre::Result<()> {
    let args = flags::Args::parse();

    setup_logging(args.verbose);

    // let saldo = match &args.saldo {
    //     None => None,
    //     Some(saldo_file) => Some(SaldoLexicon::new(saldo_file)?),
    // };
    match args.cmd {
        flags::SaldoWsdCmd::AspServer(cmd) => {
            with_extra_thread(
                "AspServer",
                stdx::thread::ThreadIntent::LatencySensitive,
                run_server,
            )?;
        }
        flags::SaldoWsdCmd::Batch(cmd) => cmd.run()?,
    }

    // FIXME: ratios not supported yet
    // if args.for_lemma.is_some() {
    //     todo!("printRatios(ratios)");
    // }

    // TODO split into chunks and use thread pool
    Ok(())
}

fn setup_logging(level: u8) {
    let log_level = match level {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    env_logger::builder().filter_level(log_level).init();
    // fastrace::set_reporter(ConsoleReporter, Config::default());
}

const STACK_SIZE: usize = 1024 * 1024 * 8;

/// Parts of rust-analyzer can use a lot of stack space, and some operating systems only give us
/// 1 MB by default (eg. Windows), so this spawns a new thread with hopefully sufficient stack
/// space.
fn with_extra_thread(
    thread_name: impl Into<String>,
    thread_intent: stdx::thread::ThreadIntent,
    f: impl FnOnce() -> eyre::Result<()> + Send + 'static,
) -> eyre::Result<()> {
    let handle = stdx::thread::Builder::new(thread_intent, thread_name)
        .stack_size(STACK_SIZE)
        .spawn(f)?;

    handle.join()?;

    Ok(())
}

fn run_server() -> eyre::Result<()> {
    log::info!("server version {} will start", saldowsd::version());
    // todo!()
    let (connection, io_threads) = Connection::stdio();

    let (initialize_id, initalize_params) = match connection.initialize_start() {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    log::info!("InitializeParams: {}", initalize_params);
    let asp_types::InitializeParams { client_info, .. } =
        from_json::<asp_types::InitializeParams>("InitializeParams", &initalize_params)?;

    if let Some(client_info) = &client_info {
        log::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.as_deref().unwrap_or_default()
        );
    }

    let config = Config::new();
    let server_capabilities = saldowsd::server_capabilities();
    let initialize_result = asp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(asp_types::ServerInfo {
            name: String::from("saldowsd"),
            version: Some(saldowsd::version().to_string()),
        }),
        offset_encoding: None,
    };

    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    if let Err(e) = connection.initialize_finish(initialize_id, initialize_result) {
        if e.channel_is_disconnected() {
            io_threads.join()?;
        }
        return Err(e.into());
    }

    // If the io_threads have an error, there's usually an error on the main
    // loop too because the channels are closed. Ensure we report both errors.
    match (saldowsd::main_loop(config, connection), io_threads.join()) {
        (Err(loop_e), Err(join_e)) => eyre::bail!("{loop_e}\n{join_e}"),
        (Ok(_), Err(join_e)) => eyre::bail!("{join_e}"),
        (Err(loop_e), Ok(_)) => eyre::bail!("{loop_e}"),
        (Ok(_), Ok(_)) => {}
    }

    log::info!("server did shut down");
    Ok(())
}
