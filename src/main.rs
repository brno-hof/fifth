mod file_io;
mod interpreter;

use std::env;
use std::io::{self, Write};
use std::process;

use interpreter::{ParseError, Program, RuntimeError};

struct Config {
    filename: String,
    stack_size: usize,
    verbose: bool,
    step: bool,
}

fn main() {
    let config = match parse_args() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("Usage: program [OPTIONS] <filename>");
            eprintln!("Options:");
            eprintln!("  --stack-size=<size>  Set stack size (default: 256)");
            eprintln!("  -v, --verbose        Print every step");
            eprintln!("  -s, --step           Wait for user input after every step");
            process::exit(1);
        }
    };

    match run(config) {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut config = Config {
        filename: String::new(),
        stack_size: 256,
        verbose: false,
        step: false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-v" | "--verbose" => {
                config.verbose = true;
                i += 1;
            }
            "-s" | "--step" => {
                config.step = true;
                i += 1;
            }
            arg if arg.starts_with("--stack-size=") => {
                let size_str = &arg["--stack-size=".len()..];
                config.stack_size = size_str
                    .parse()
                    .map_err(|_| format!("Invalid stack size: {}", size_str))?;
                i += 1;
            }
            arg if arg.starts_with("-") => {
                return Err(format!("Unknown option: {}", arg));
            }
            _ => {
                if config.filename.is_empty() {
                    config.filename = args[i].clone();
                } else {
                    return Err("Multiple filenames specified".to_string());
                }
                i += 1;
            }
        }
    }

    if config.filename.is_empty() {
        return Err("No filename specified".to_string());
    }

    Ok(config)
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = file_io::read_file_to_string(&config.filename)?;

    let mut program = Program::new(&content, config.stack_size);

    match program.parse() {
        Ok(_) => (),
        Err(err) => {
            match err {
                ParseError::InvalidArgument(arg, line) => {
                    eprintln!("Parse error at line {}: Invalid argument '{}'", line, arg);
                }
                ParseError::MissingArgument(token, line) => {
                    eprintln!(
                        "Parse error at line {}: Missing argument for '{}'",
                        line, token
                    );
                }
                ParseError::DuplicateLabel(label, line) => {
                    eprintln!("Parse error at line {}: Duplicate label '{}'", line, label);
                }
                ParseError::InvalidCall(label, line) => {
                    eprintln!(
                        "Parse error at line {}: Call to undefined label '{}'",
                        line, label
                    );
                }
                ParseError::ElseWithoutIfStatement(token) => {
                    eprintln!("Parse error at line {}: ELSE without IF", token.line_number);
                }
                ParseError::ThenWithoutIfStatement(token) => {
                    eprintln!("Parse error at line {}: THEN without IF", token.line_number);
                }
                ParseError::TooManyElseStatements(token) => {
                    eprintln!(
                        "Parse error at line {}: Multiple ELSE statements for single IF",
                        token.line_number
                    );
                }
            }
            process::exit(1);
        }
    }

    while !program.halted {
        if config.verbose || config.step {
            let current_token = &program.tokens[program.pc];
            println!("Stack: {:?}", program.stack);
            println!(
                "Line {}: {}",
                current_token.line_number,
                current_token.token.to_string()
            );

            if config.step {
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
        }

        match program.step() {
            Ok(_) => (),
            Err(err) => {
                match err {
                    RuntimeError::StackOverflow(token) => {
                        eprintln!(
                            "Runtime error at line {}: Stack overflow",
                            token.line_number
                        );
                    }
                    RuntimeError::StackUnderflow(token) => {
                        eprintln!(
                            "Runtime error at line {}: Stack underflow",
                            token.line_number
                        );
                    }
                    RuntimeError::InvalidLabel(token) => {
                        eprintln!("Runtime error at line {}: Invalid label", token.line_number);
                    }
                    RuntimeError::CallStackUnderflow(token) => {
                        eprintln!(
                            "Runtime error at line {}: Call stack underflow",
                            token.line_number
                        );
                    }
                    RuntimeError::UnclosedIfStatement(token) => {
                        eprintln!(
                            "Runtime error at line {}: Unclosed IF statement",
                            token.line_number
                        );
                    }
                }
                process::exit(1);
            }
        }
    }

    if config.verbose || config.step {
        println!("Program halted.");
        println!("Final stack: {:?}", program.stack);
    }

    Ok(())
}
