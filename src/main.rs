use std::{fs, process};

use clap::{Parser, Subcommand};
use cookie_parser::{parse_cookie_string, parse_set_cookie};

#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ParseCookie {
        #[arg(short, long)]
        file: String
    },
    ParseSetCookie {
        #[arg(short, long)]
        file: String
    },
    Credits,
    Help,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ParseCookie { file } => {
            let file_contents = fs::read_to_string(file).unwrap_or_else(|err| {
                eprintln!("Error reading from file: {}", err);
                process::exit(1)
            });

            let file_cookies = parse_cookie_string(&file_contents);

            match file_cookies {
                Ok(result) => {
                    println!("Parsed cookies:");
                    for cookie in result {
                        println!("{}: {}", cookie.name, cookie.value)
                    }
                }
                Err(err) => {
                    eprintln!("Error parsing: {}", err);
                    process::exit(1)
                }
            }
        },
        Commands::ParseSetCookie { file } => {
            let file_contents = fs::read_to_string(file).unwrap_or_else(|err| {
                eprintln!("Error reading from file: {}", err);
                process::exit(1)
            });

            let set_cookie = parse_set_cookie(&file_contents);

            match set_cookie {
                Ok(result) => {
                    println!("Parsed Set-Cookie:");
                    println!("Cookie: {}={}", result.pair.name, result.pair.value);
                    println!("Secure: {}", result.secure);
                    println!("HttpOnly: {}", result.http_only);
                    
                    if let Some(max_age) = &result.max_age {
                        println!("Max-Age: {}", max_age);
                    }
                    
                    if let Some(domain) = &result.domain {
                        println!("Domain: {}", domain);
                    }
                    
                    if let Some(expires) = &result.expires {
                        println!("Expires: {}", expires);
                    }
                    
                    if let Some(path) = &result.path {
                        println!("Path: {}", path);
                    }
                    
                    if !result.extensions.is_empty() {
                        println!("Extensions:");
                        for ext in &result.extensions {
                            println!("  - {}", ext);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error parsing: {}", err);
                    process::exit(1)
                }
            }
        },
        Commands::Credits => {
            println!("Cookie/Set-Cookie Parser CLI");
            println!("Version: 1.0");
            println!("Author: Artem Tarasenko <artem.tarasenko@ukma.edu.ua,shabashab.04@gmail.com>");
        },
        Commands::Help => {
            println!("Cookie/Set-Cookie Parser CLI");
            println!();
            println!("USAGE:");
            println!("\t cookie_parser <COMMAND>");
            println!();
            println!("COMMANDS:");
            println!("\tparse-cookie Parses Cookie header contents.");
            println!("\t\tOptions:");
            println!("\t\t\t--file,-f <FILE>     File with contents that should be parsed.");
            println!();
            println!("\tparse-set-cookie Parses Set-Cookie header contents.");
            println!("\t\tOptions:");
            println!("\t\t\t--file,-f <FILE>     File with contents that should be parsed.");
            println!();
            println!("\tcredits Show credits.");
            println!();
            println!("\thelp    Show help.");
            println!();
        }
    }
}
