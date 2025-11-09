use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use triggerlang::{parse_triggers, parse_triggers_to_ast};

#[derive(Parser)]
#[command(name = "triggerlang")]
#[command(author = "Lys Taras")]
#[command(version = "v1.0.0")]
#[command(about = "A parser for custom trigger language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(value_name = "FILE")]
        file: PathBuf,

        #[arg(short, long)]
        ast: bool,

        #[arg(short, long)]
        verbose: bool,
    },

    Credits,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file, ast, verbose } => {
            if verbose {
                println!("Parsing file...");
                println!("");
                println!("File: {}", file.display());
            }

            match fs::read_to_string(&file) {
                Ok(content) => {
                    if verbose {
                        println!("File content was readed successfully");
                        println!("");
                    }

                    match parse_triggers(&content) {
                        Ok(_) => {
                            println!("First syntax validation passed");

                            match parse_triggers_to_ast(&content) {
                                Ok(ast_tree) => {
                                    println!("Generated AST");
                                    println!("Found {} triggers", ast_tree.triggers.len());
                                    println!("");

                                    if ast {
                                        println!("AST tree");
                                        println!("");

                                        for (idx, trigger) in ast_tree.triggers.iter().enumerate() {
                                            println!("Trigger #{}", idx + 1);
                                            println!("- Name: {}", trigger.name);
                                            println!("- Event: {}", trigger.event_type);
                                            println!("- Description: \"{}\"", trigger.description);
                                            println!("");

                                            if let Some(condition) = &trigger.condition {
                                                println!("- Condition:");
                                                print_condition_tree(condition, 1);
                                            } else {
                                                println!("- Condition: None");
                                            }

                                            println!("- Actions ({}):", trigger.actions.len());
                                            for (i, action) in trigger.actions.iter().enumerate() {
                                                println!(" - {}", action);
                                            }
                                            println!("");
                                        }
                                    } else {
                                        println!("Triggers found:");
                                        for (idx, trigger) in ast_tree.triggers.iter().enumerate() {
                                            println!("  {}. {} ({})", idx + 1, trigger.name, trigger.event_type);
                                        }
                                        println!("");
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to generate AST");
                                    eprintln!("Error: {}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Syntax validation failed");
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read file: {}", file.display());
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Credits => {
            println!("triggerlang v1.0.0");
            println!("======");
            println!("A parser for custom trigger language. Inspired by Battlemetrics trigger system.");
            println!("======");
            println!("Author: Lys Taras");
            println!("Repository: https://github.com/1KINGO1/triggerlang");
            println!("======");
            println!("Built with:");
            println!("Rust");
            println!("Pest Parser (v2.8.3)");
            println!("Clap CLI (v4.5.51)");
            println!("======");
            println!("Example:");
            println!("trigger WelcomePlayer {{");
            println!(" on: player_join");
            println!(" description: \"Welcome new players\"");
            println!(" condition: player.is_new == true");
            println!(" action: send_message(\"Welcome!\")");
            println!("}};");
        }
    }
}

fn print_condition_tree(expr: &triggerlang::Expr, depth: usize) {
    let indent = " ".repeat(depth);

    match expr {
        triggerlang::Expr::And(left, right) => {
            println!("{} AND", indent);
            print_condition_tree(left, depth + 1);
            print_condition_tree(right, depth + 1);
        }
        triggerlang::Expr::Or(left, right) => {
            println!("{} OR", indent);
            print_condition_tree(left, depth + 1);
            print_condition_tree(right, depth + 1);
        }
        triggerlang::Expr::Not(inner) => {
            println!("{} NOT", indent);
            print_condition_tree(inner, depth + 1);
        }
        triggerlang::Expr::Comparison(comp) => {
            println!("{} Comparison: {} {} {}", indent, comp.left, comp.operator, comp.right);
        }
        triggerlang::Expr::FuncCall(func) => {
            println!("{} FuncCall: {}", indent, func);
        }
        triggerlang::Expr::Ident(id) => {
            println!("{} Identifier: {}", indent, id);
        }
        triggerlang::Expr::Parenthesized(inner) => {
            println!("{} Parenthesized:", indent);
            print_condition_tree(inner, depth + 1);
        }
    }
}
