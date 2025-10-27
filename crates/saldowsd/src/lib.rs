mod asp;
pub mod cli;
pub mod config;
mod global_state;
mod main_loop;
mod version;

use serde::de::DeserializeOwned;

pub use crate::{asp::capabilities::server_capabilities, version::version};

pub fn from_json<T: DeserializeOwned>(
    what: &'static str,
    json: &serde_json::Value,
) -> eyre::Result<T> {
    serde_json::from_value(json.clone())
        .map_err(|e| eyre::format_err!("Failed to deserialize {what}: {e}; {json}"))
}
