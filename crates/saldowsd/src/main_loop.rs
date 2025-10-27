use std::fmt;

use asp_server::Connection;
use crossbeam_channel::{Receiver, select};

use crate::{config::Config, global_state::GlobalState};

pub fn main_loop(config: Config, connection: Connection) -> eyre::Result<()> {
    log::info!("inital config: {:#?}", config);

    GlobalState::new(connection.sender, config).run(connection.receiver)
}

enum Event {
    Lsp(asp_server::Message),
    // Task(Task),
    // QueuedTask(QueuedTask),
    // Vfs(vfs::loader::Message),
    // Flycheck(FlycheckMessage),
    // TestResult(CargoTestMessage),
    // DiscoverProject(DiscoverProjectMessage),
    // FetchWorkspaces(FetchWorkspaceRequest),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Lsp(_) => write!(f, "Event::Asp"),
            // Event::Task(_) => write!(f, "Event::Task"),
            // Event::Vfs(_) => write!(f, "Event::Vfs"),
            // Event::Flycheck(_) => write!(f, "Event::Flycheck"),
            // Event::QueuedTask(_) => write!(f, "Event::QueuedTask"),
            // Event::TestResult(_) => write!(f, "Event::TestResult"),
            // Event::DiscoverProject(_) => write!(f, "Event::DiscoverProject"),
            // Event::FetchWorkspaces(_) => write!(f, "Event::SwitchWorkspaces"),
        }
    }
}
impl GlobalState {
    fn run(mut self, inbox: Receiver<asp_server::Message>) -> eyre::Result<()> {
        // self.update_status_or_notify();

        // if self.config.did_save_text_document_dynamic_registration() {
        //     let additional_patterns = self
        //         .config
        //         .discover_workspace_config()
        //         .map(|cfg| cfg.files_to_watch.clone().into_iter())
        //         .into_iter()
        //         .flatten()
        //         .map(|f| format!("**/{f}"));
        //     self.register_did_save_capability(additional_patterns);
        // }

        // if self.config.discover_workspace_config().is_none() {
        //     self.fetch_workspaces_queue.request_op(
        //         "startup".to_owned(),
        //         FetchWorkspaceRequest {
        //             path: None,
        //             force_crate_graph_reload: false,
        //         },
        //     );
        //     if let Some((
        //         cause,
        //         FetchWorkspaceRequest {
        //             path,
        //             force_crate_graph_reload,
        //         },
        //     )) = self.fetch_workspaces_queue.should_start_op()
        //     {
        //         self.fetch_workspaces(cause, path, force_crate_graph_reload);
        //     }
        // }

        while let Ok(event) = self.next_event(&inbox) {
            let Some(event) = event else {
                eyre::bail!("client exited without proper shutdown sequence");
            };
            if matches!(
                &event,
                Event::Lsp(asp_server::Message::Notification(Notification { method, .. }))
                if method == asp_types::notification::Exit::METHOD
            ) {
                return Ok(());
            }
            self.handle_event(event);
        }

        Err(eyre::anyhow!(
            "A receiver has been dropped, something panicked!"
        ))
    }

    fn next_event(
        &mut self,
        inbox: &Receiver<asp_server::Message>,
    ) -> Result<Option<Event>, crossbeam_channel::RecvError> {
        // Make sure we reply to formatting requests ASAP so the editor doesn't block
        // if let Ok(task) = self.fmt_pool.receiver.try_recv() {
        //     return Ok(Some(Event::Task(task)));
        // }

        select! {
                recv(inbox) -> msg =>
                    return Ok(msg.ok().map(Event::Lsp)),

                recv(self.task_pool.receiver) -> task =>
                    task.map(Event::Task),

                recv(self.deferred_task_queue.receiver) -> task =>
                    task.map(Event::QueuedTask),

                recv(self.fmt_pool.receiver) -> task =>
                    task.map(Event::Task),

                recv(self.loader.receiver) -> task =>
                    task.map(Event::Vfs),

                recv(self.flycheck_receiver) -> task =>
                    task.map(Event::Flycheck),

                recv(self.test_run_receiver) -> task =>
                    task.map(Event::TestResult),

                recv(self.discover_receiver) -> task =>
                    task.map(Event::DiscoverProject),

                recv(self.fetch_ws_receiver.as_ref().map_or(&never(), |(chan, _)| chan)) -> _instant => {
                    Ok(Event::FetchWorkspaces(self.fetch_ws_receiver.take().unwrap().1))
                },
            }
            .map(Some)
    }

    fn handle_event(&mut self, event: Event) {
        let loop_start = Instant::now();
        let _p = tracing::info_span!("GlobalState::handle_event", event = %event).entered();

        let event_dbg_msg = format!("{event:?}");
        tracing::debug!(?loop_start, ?event, "handle_event");
        if tracing::enabled!(tracing::Level::INFO) {
            let task_queue_len = self.task_pool.handle.len();
            if task_queue_len > 0 {
                tracing::info!("task queue len: {}", task_queue_len);
            }
        }

        let was_quiescent = self.is_quiescent();
        match event {
            Event::Lsp(msg) => match msg {
                asp_server::Message::Request(req) => self.on_new_request(loop_start, req),
                asp_server::Message::Notification(not) => self.on_notification(not),
                asp_server::Message::Response(resp) => self.complete_request(resp),
            },
            Event::QueuedTask(task) => {
                let _p = tracing::info_span!("GlobalState::handle_event/queued_task").entered();
                self.handle_queued_task(task);
                // Coalesce multiple task events into one loop turn
                while let Ok(task) = self.deferred_task_queue.receiver.try_recv() {
                    self.handle_queued_task(task);
                }
            }
            Event::Task(task) => {
                let _p = tracing::info_span!("GlobalState::handle_event/task").entered();
                let mut prime_caches_progress = Vec::new();

                self.handle_task(&mut prime_caches_progress, task);
                // Coalesce multiple task events into one loop turn
                while let Ok(task) = self.task_pool.receiver.try_recv() {
                    self.handle_task(&mut prime_caches_progress, task);
                }

                for progress in prime_caches_progress {
                    let (state, message, fraction, title);
                    match progress {
                        PrimeCachesProgress::Begin => {
                            state = Progress::Begin;
                            message = None;
                            fraction = 0.0;
                            title = "Indexing";
                        }
                        PrimeCachesProgress::Report(report) => {
                            state = Progress::Report;
                            title = report.work_type;

                            message = match &*report.crates_currently_indexing {
                                [crate_name] => Some(format!(
                                    "{}/{} ({})",
                                    report.crates_done,
                                    report.crates_total,
                                    crate_name.as_str(),
                                )),
                                [crate_name, rest @ ..] => Some(format!(
                                    "{}/{} ({} + {} more)",
                                    report.crates_done,
                                    report.crates_total,
                                    crate_name.as_str(),
                                    rest.len()
                                )),
                                _ => None,
                            };

                            fraction = Progress::fraction(report.crates_done, report.crates_total);
                        }
                        PrimeCachesProgress::End { cancelled } => {
                            state = Progress::End;
                            message = None;
                            fraction = 1.0;
                            title = "Indexing";

                            self.analysis_host.raw_database_mut().trigger_lru_eviction();
                            self.prime_caches_queue.op_completed(());
                            if cancelled {
                                self.prime_caches_queue
                                    .request_op("restart after cancellation".to_owned(), ());
                            }
                        }
                    };

                    self.report_progress(
                        title,
                        state,
                        message,
                        Some(fraction),
                        Some("rustAnalyzer/cachePriming".to_owned()),
                    );
                }
            }
            Event::Vfs(message) => {
                let _p = tracing::info_span!("GlobalState::handle_event/vfs").entered();
                self.handle_vfs_msg(message);
                // Coalesce many VFS event into a single loop turn
                while let Ok(message) = self.loader.receiver.try_recv() {
                    self.handle_vfs_msg(message);
                }
            }
            Event::Flycheck(message) => {
                let _p = tracing::info_span!("GlobalState::handle_event/flycheck").entered();
                self.handle_flycheck_msg(message);
                // Coalesce many flycheck updates into a single loop turn
                while let Ok(message) = self.flycheck_receiver.try_recv() {
                    self.handle_flycheck_msg(message);
                }
            }
            Event::TestResult(message) => {
                let _p = tracing::info_span!("GlobalState::handle_event/test_result").entered();
                self.handle_cargo_test_msg(message);
                // Coalesce many test result event into a single loop turn
                while let Ok(message) = self.test_run_receiver.try_recv() {
                    self.handle_cargo_test_msg(message);
                }
            }
            Event::DiscoverProject(message) => {
                self.handle_discover_msg(message);
                // Coalesce many project discovery events into a single loop turn.
                while let Ok(message) = self.discover_receiver.try_recv() {
                    self.handle_discover_msg(message);
                }
            }
            Event::FetchWorkspaces(req) => self
                .fetch_workspaces_queue
                .request_op("project structure change".to_owned(), req),
        }
        let event_handling_duration = loop_start.elapsed();
        let (state_changed, memdocs_added_or_removed) = if self.vfs_done {
            if let Some(cause) = self.wants_to_switch.take() {
                self.switch_workspaces(cause);
            }
            (self.process_changes(), self.mem_docs.take_changes())
        } else {
            (false, false)
        };

        if self.is_quiescent() {
            let became_quiescent = !was_quiescent;
            if became_quiescent {
                if self.config.check_on_save(None)
                    && self.config.flycheck_workspace(None)
                    && !self.fetch_build_data_queue.op_requested()
                {
                    // Project has loaded properly, kick off initial flycheck
                    self.flycheck
                        .iter()
                        .for_each(|flycheck| flycheck.restart_workspace(None));
                }
                if self.config.prefill_caches() {
                    self.prime_caches_queue
                        .request_op("became quiescent".to_owned(), ());
                }
            }

            let client_refresh = became_quiescent || state_changed;
            if client_refresh {
                // Refresh semantic tokens if the client supports it.
                if self.config.semantic_tokens_refresh() {
                    self.semantic_tokens_cache.lock().clear();
                    self.send_request::<asp_types::request::SemanticTokensRefresh>((), |_, _| ());
                }

                // Refresh code lens if the client supports it.
                if self.config.code_lens_refresh() {
                    self.send_request::<asp_types::request::CodeLensRefresh>((), |_, _| ());
                }

                // Refresh inlay hints if the client supports it.
                if self.config.inlay_hints_refresh() {
                    self.send_request::<asp_types::request::InlayHintRefreshRequest>((), |_, _| ());
                }

                if self.config.diagnostics_refresh() {
                    self.send_request::<asp_types::request::WorkspaceDiagnosticRefresh>(
                        (),
                        |_, _| (),
                    );
                }
            }

            let project_or_mem_docs_changed =
                became_quiescent || state_changed || memdocs_added_or_removed;
            if project_or_mem_docs_changed
                && !self.config.text_document_diagnostic()
                && self.config.publish_diagnostics(None)
            {
                self.update_diagnostics();
            }
            if project_or_mem_docs_changed && self.config.test_explorer() {
                self.update_tests();
            }
        }

        if let Some(diagnostic_changes) = self.diagnostics.take_changes() {
            for file_id in diagnostic_changes {
                let uri = file_id_to_url(&self.vfs.read().0, file_id);
                let version = from_proto::vfs_path(&uri)
                    .ok()
                    .and_then(|path| self.mem_docs.get(&path).map(|it| it.version));

                let diagnostics = self
                    .diagnostics
                    .diagnostics_for(file_id)
                    .cloned()
                    .collect::<Vec<_>>();
                self.publish_diagnostics(uri, version, diagnostics);
            }
        }

        if (self.config.cargo_autoreload_config(None)
            || self.config.discover_workspace_config().is_some())
            && let Some((
                cause,
                FetchWorkspaceRequest {
                    path,
                    force_crate_graph_reload,
                },
            )) = self.fetch_workspaces_queue.should_start_op()
        {
            self.fetch_workspaces(cause, path, force_crate_graph_reload);
        }

        if !self.fetch_workspaces_queue.op_in_progress() {
            if let Some((cause, ())) = self.fetch_build_data_queue.should_start_op() {
                self.fetch_build_data(cause);
            } else if let Some((cause, (change, paths))) =
                self.fetch_proc_macros_queue.should_start_op()
            {
                self.fetch_proc_macros(cause, change, paths);
            }
        }

        if let Some((cause, ())) = self.prime_caches_queue.should_start_op() {
            self.prime_caches(cause);
        }

        self.update_status_or_notify();

        let loop_duration = loop_start.elapsed();
        if loop_duration > Duration::from_millis(100) && was_quiescent {
            tracing::warn!(
                "overly long loop turn took {loop_duration:?} (event handling took {event_handling_duration:?}): {event_dbg_msg}"
            );
            self.poke_rust_analyzer_developer(format!(
                    "overly long loop turn took {loop_duration:?} (event handling took {event_handling_duration:?}): {event_dbg_msg}"
                ));
        }
    }
}
