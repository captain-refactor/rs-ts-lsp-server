use std::collections::HashMap;
use std::sync::Arc;

use log::{error, info, warn};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result as JsonResult;
use tower_lsp::lsp_types::{
    CompletionOptions, CompletionParams, CompletionResponse, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, Hover,
    HoverParams, HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams,
    MessageType, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp::{
    Client, ClientSocket, LanguageServer, LspService, Server as LspServer, async_trait,
};

type SharedState = Arc<Mutex<ServerState>>;

#[derive(Default)]
struct ServerState {
    documents: HashMap<String, String>,
}

pub struct Backend {
    client: Client,
    state: SharedState,
}

impl Backend {
    fn new(client: Client, state: SharedState) -> Self {
        Self { client, state }
    }

    async fn log(&self, message: &str) {
        info!("{}", message);
        if let Err(error) = self
            .client
            .log_message(MessageType::INFO, message.to_string())
            .await
        {
            error!("Failed to send client log message: {error}");
        }
    }
}

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> JsonResult<InitializeResult> {
        info!("Received initialize request: {params:?}");
        self.log("Language server initialization started.").await;

        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            completion_provider: Some(CompletionOptions::default()),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            ..ServerCapabilities::default()
        };

        let result = InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            ..InitializeResult::default()
        };

        Ok(result)
    }

    async fn initialized(&self, params: InitializedParams) {
        info!("Client initialized: {params:?}");
        self.log("Language server initialized.").await;
    }

    async fn shutdown(&self) -> JsonResult<()> {
        info!("Shutdown requested");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document;
        let uri = text_document.uri.to_string();
        info!("Opened document: {uri}");
        let mut state = self.state.lock().await;
        state.documents.insert(uri.clone(), text_document.text);

        self.log(&format!("Document opened: {uri}")).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams {
            text_document,
            content_changes,
        } = params;
        let uri = text_document.uri.to_string();
        info!(
            "Change event for document {uri} with {} change(s)",
            content_changes.len()
        );

        let mut state = self.state.lock().await;
        if let Some(change) = content_changes.into_iter().last() {
            state.documents.insert(uri.clone(), change.text);
        } else {
            warn!("No change content supplied for {uri}");
        }

        self.log(&format!("Document changed: {uri}")).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        info!("Saved document: {uri}");
        self.log(&format!("Document saved: {uri}")).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        info!("Closed document: {uri}");
        let mut state = self.state.lock().await;
        state.documents.remove(&uri);

        self.log(&format!("Document closed: {uri}")).await;
    }

    async fn hover(&self, params: HoverParams) -> JsonResult<Option<Hover>> {
        info!("Hover request: {params:?}");
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> JsonResult<Option<CompletionResponse>> {
        info!("Completion request: {params:?}");
        Ok(Some(CompletionResponse::Array(Vec::new())))
    }
}

pub struct Server {
    state: SharedState,
}

impl Server {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServerState::default())),
        }
    }

    fn create_service(&self) -> (LspService<Backend>, ClientSocket) {
        let state = Arc::clone(&self.state);
        LspService::new(move |client| Backend::new(client, Arc::clone(&state)))
    }

    pub async fn run() -> JsonResult<()> {
        let server = Self::new();
        let (service, socket) = server.create_service();
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        info!("Starting LSP server");
        LspServer::new(stdin, stdout, socket).serve(service).await
    }
}
