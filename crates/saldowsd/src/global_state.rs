use std::{sync::Arc, time::Instant};

use asp_server::ReqQueue;
use crossbeam_channel::Sender;

use crate::config::Config;

pub(crate) type ReqHandler = fn(&mut GlobalState, asp_server::Response);
type ReqQueue = asp_server::ReqQueue<(String, Instant), ReqHandler>;

/// `GlobalState` is the primary mutable state of the language server
///
/// Note that this struct has more than one impl in various modules!
pub(crate) struct GlobalState {
    sender: Sender<asp_server::Message>,
    req_queue: ReqQueue,

    pub(crate) config: Arc<Config>,

    // status
    pub(crate) shutdown_requested: bool,
}

impl GlobalState {
    pub(crate) fn new(sender: Sender<asp_server::Message>, config: Config) -> GlobalState {
        GlobalState {
            sender,
            req_queue: ReqQueue::default(),
            config: Arc::new(config),
            shutdown_requested: false,
        }
    }
}
