// ========================= Actual Protocol =========================

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    /// The process Id of the parent process that started
    /// the server. Is null if the process has not been started by another process.
    /// If the parent process is not alive then the server should exit (see exit notification) its process.
    pub process_id: Option<u32>,

    /// User provided initialization options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_options: Option<serde_json::Value>,

    // /// The capabilities provided by the client (editor or tool)
    // pub capabilities: ClientCapabilities,

    // /// The initial trace setting. If omitted trace is disabled ('off').
    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub trace: Option<TraceValue>,

    // /// The workspace folders configured in the client when the server starts.
    // /// This property is only available if the client supports workspace folders.
    // /// It can be `null` if the client supports workspace folders but none are
    // /// configured.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub workspace_folders: Option<Vec<WorkspaceFolder>>,
    /// Information about the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_info: Option<ClientInfo>,

    /// The locale the client is currently showing the user interface
    /// in. This must not necessarily be the locale of the operating
    /// system.
    ///
    /// Uses IETF language tags as the value's syntax
    /// (See <https://en.wikipedia.org/wiki/IETF_language_tag>)
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    // /// The LSP server may report about initialization progress to the client
    // /// by using the following work done token if it was passed by the client.
    // #[serde(flatten)]
    // pub work_done_progress_params: WorkDoneProgressParams,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ClientInfo {
    /// The name of the client as defined by the client.
    pub name: String,
    /// The client's version as defined by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    /// The capabilities the language server provides.
    pub capabilities: ServerCapabilities,

    /// Information about the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,

    /// Unofficial UT8-offsets extension.
    ///
    /// See https://clangd.llvm.org/extensions.html#utf-8-offsets.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "proposed")]
    pub offset_encoding: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,
    /// The servers's version as defined by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct InitializedParams {}
#[derive(Debug, PartialEq, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {}
