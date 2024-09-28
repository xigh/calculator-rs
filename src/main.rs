mod compute;
mod parser;
mod tokenizer;

use std::env::args;
use tracing::Level;

fn main() {
    let arg0 = args().next().unwrap();
    let (options, inputs): (Vec<String>, Vec<String>) =
        args().skip(1).partition(|input| input.starts_with("--"));

    let mut show_tokens = false;
    let mut show_ast = false;
    let mut log_level = Level::ERROR;

    for option in options {
        match option.split('=').nth(0).unwrap() {
            "--help" => {
                println!("Usage: {} <options> <expression>", arg0);
                println!("Options:");
                println!("  --help\tShow this help message and exit");
                println!("  --version\tShow version information and exit");
                println!("  --show-tokens\tShow tokens");
                println!("  --show-ast\tShow AST");
                println!(
                    "  --log-level\tSet log level (ERROR*, WARN, INFO, DEBUG, TRACE)"
                );
                return;
            }
            "--version" => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--show-tokens" => {
                show_tokens = true;
            }
            "--show-ast" => {
                show_ast = true;
            }
            "--log-level" => {
                log_level = match option.split('=').nth(1) {
                    Some(level) => match level.to_lowercase().as_str() {
                        "info" => Level::INFO,
                        "debug" => Level::DEBUG,
                        "warn" => Level::WARN,
                        "error" => Level::ERROR,
                        "trace" => Level::TRACE,
                        _ => {
                            eprintln!("unknown log level: {}", level);
                            return;
                        }
                    },
                    _ => {
                        eprintln!("invalid log level");
                        return;
                    }
                };
            }
            _ => unreachable!(),
        }
    }

    tracing_subscriber::fmt().with_max_level(log_level).init();

    for input in inputs {
        let tokens = tokenizer::tokenize(input.as_str());
        match tokens {
            Ok(tokens) => {
                if show_tokens {
                    println!("{:?}", tokens);
                }
                let ast = parser::parse(tokens);
                match ast {
                    Ok(ast) => {
                        if show_ast {
                            println!("{:?}", ast);
                        }
                        match compute::execute(ast) {
                            Ok(result) => println!("{} -> {}", input, result),
                            Err(e) => println!("got error: {}", e),
                        }
                    }
                    Err(e) => println!("parsing failed: {}", e),
                }
            }
            Err(e) => println!("tokenizing failed: {}", e),
        }
    }
}
