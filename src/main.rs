mod lexer;
mod token;
// mod parser;
// mod ast;
// mod analyzer;
mod server;

use log::error;

// use lexer::Lexer;
// use parser::Parser;
// use ast::AST;
// use analyzer::Analyzer;
// use server::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(error) = server::Server::run().await {
        error!("Language server exited with error: {error}");
    }
}
