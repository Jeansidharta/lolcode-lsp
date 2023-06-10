use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {},
    Ast { filename: PathBuf },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run {} => {
            lolcode_lsp::start_lsp().await;
        }
        Commands::Ast { filename } => {
            let code = match std::fs::read_to_string(filename) {
                Err(_) => {
                    println!("Error when reading file");
                    return;
                }
                Ok(val) => val,
            };
            let ast = lolcode_ast::tokenize_and_parse(code);
            println!("{:#?}", ast);
        }
    }
}
