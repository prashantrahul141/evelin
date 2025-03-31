mod ast;
mod cli;
mod lexer;
mod parser;
mod qbe_backend;
mod qbe_generator;
mod token;
mod utils;

use std::fs;

use lexer::Lexer;
use log::error;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() {
    let options = cli::init();
    match options.file {
        // file mode.
        cli::InFile::File(f) => {
            let in_src = fs::read_to_string(f).unwrap();
            let mut lexer = lexer::Lexer::new(&in_src);
            lexer.start();
            let mut parser = parser::Parser::new(lexer.tokens());
            parser.parse();
            dbg!(parser.stmts);
        }
        // repl mode.
        cli::InFile::Stdin => {
            let mut rl = DefaultEditor::new().unwrap();
            loop {
                let readline = rl.readline(">>> ");
                match readline {
                    Ok(line) => {
                        let _ = rl.add_history_entry(line.as_str());
                        let mut lexer = Lexer::new(&line);
                        lexer.start();
                        let mut parser = parser::Parser::new(lexer.tokens());
                        parser.parse();
                        dbg!(parser.stmts);
                    }
                    Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                        println!("Interrupted");
                        break;
                    }
                    Err(err) => {
                        die!("Failed to readline : {:?}", err);
                    }
                }
            }
        }
    };
}
