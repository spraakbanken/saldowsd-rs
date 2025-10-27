use std::io;

use eyre::bail;
use wsd_application::{
    SourceFormat, TabFormat, VectorWSD, VectorWSDConfig,
    wsd_application::{DisambiguateOptions, disambiguate_sentences},
};

use crate::cli::flags;

impl flags::Batch {
    pub fn run(self) -> eyre::Result<()> {
        let args = self.args;
        // FIXME: use new_as_shared if more apps are included
        let wsd = match args.app_name {
            flags::AppNames::VectorWSD {
                decay,
                s1_prior,
                context_width,
                sv_file,
                cv_file,
            } => VectorWSD::new(
                &sv_file,
                &cv_file,
                VectorWSDConfig {
                    decay,
                    s1prior: s1_prior,
                    context_width,
                },
            )?,
        };
        // let wsd = make_wsd_application(saldo.as_ref(), &args.app_name, &argv)?;

        // FIXME: eval not supported yet
        // if args.format == Format::Eval {
        //     evaluate(wsd, &args.eval_lemmas.unwrap(), &args.eval_key.unwrap());
        //     return Ok(());
        // }

        // FIXME: ratios not supported yet
        // let mut ratios = None;
        // if args.for_lemma.is_some() {
        //     todo!("ratios is not yet supported");
        //     // todo!("ratios = Some(HashMap::new())");
        // }

        let mut stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        let format: Box<dyn SourceFormat> = if args.format == flags::Format::Sbxml {
            bail!("sbxml format is not yet supported");
        } else {
            Box::new(TabFormat::default())
        };
        disambiguate_sentences(
            wsd,
            &mut stdin,
            &mut stdout,
            &format,
            DisambiguateOptions {
                batch_size: args.batch_size,
                max_sen: args.max_sen,
            },
        )?;
        Ok(())
    }
}
