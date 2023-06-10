use tower_lsp::jsonrpc::Result;
use tower_lsp::{lsp_types::*, Client, LanguageServer, LspService, Server};

use std::fs::write;

#[derive(Debug)]
struct Backend {
    client: Client,
}

const DEBUG_FILE_PATH: &str = "/home/sidharta/fifo";

#[allow(unused_macros)]
macro_rules! dbg_file {
    () => {
        std::fs::write(DEBUG_FILE_PATH.into(), "[{}:{}]", std::file!(), std::line!()).unwrap()
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                std::fs::write(DEBUG_FILE_PATH, format!("[{}:{}] {} = {:#?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp)
                ).unwrap();
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg_file!($val)),+,)
    };
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    #[allow(unused_must_use)]
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        // dbg_file!(&params);
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "lolcode-lsp".to_string(),
                version: Some("3.17.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Initialized!".to_string())
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(
        &self,
        DidOpenTextDocumentParams {
            text_document:
                TextDocumentItem {
                    uri: _,
                    language_id: _,
                    version: _,
                    text,
                },
        }: DidOpenTextDocumentParams,
    ) {
        dbg_file!("Open", &text);
        let ast = match lolcode_ast::tokenize_and_parse(text).map_err(|err| match err {
            lolcode_ast::lexer::TokenError::UnclosedMultilineComment(_) => {
                "Error on multiline comment"
            }
        }) {
            Err(err) => {
                dbg_file!(err);
                todo!()
            }
            Ok(ast) => ast,
        };
    }

    async fn did_change(
        &self,
        DidChangeTextDocumentParams {
            text_document: _,
            content_changes,
        }: DidChangeTextDocumentParams,
    ) {
        dbg_file!("change", content_changes);
    }

    async fn hover(
        &self,
        HoverParams {
            text_document_position_params: _,
            work_done_progress_params: _,
        }: HoverParams,
    ) -> Result<Option<Hover>> {
        dbg_file!("HOVEEER");
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("Hahaha".to_string())),
            range: None,
        }))
    }

    async fn code_action(
        &self,
        CodeActionParams {
            text_document: _,
            range: _,
            context: _,
            work_done_progress_params: _,
            partial_result_params: _,
        }: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>> {
        dbg_file!("CODE ACTION");
        Ok(Some(vec![CodeActionOrCommand::CodeAction(CodeAction {
            title: "Teste".to_string(),
            kind: Some(CodeActionKind::EMPTY),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: None,
            disabled: None,
            data: None,
        })]))
    }
}

pub async fn start_lsp() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
