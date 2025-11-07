pub mod commands;
pub mod tui;

use crate::models::{graph::Graph, node::Node};
use commands::{Cli, Commands};
use clap::Parser;

pub fn run() {
    let cli = Cli::parse();

    let mut graph = Graph::load_from_file("graph.json").unwrap_or_else(|_| Graph::new());

    match cli.command {
        Commands::AddNode { id, title, content, tags } => {
            
            let node = match id {
                Some(custom_id) => {
                    let mut node = Node::with_tags(title, content, tags);
                    node.id = custom_id;
                    node
                }
                None => Node::with_tags(title, content, tags),
            };

            graph.add_node(node);

            if let Err(e) = graph.save_to_file("graph.json") {
                eprintln!("Failed to save: {}", e);
                return;
            }
            println!(" Node added and saved.");
        }

        Commands::AddEdge { from, to } => {
            if graph.get_node(&from).is_none() {
                eprintln!(" Source node '{}' not found", from);
                return;
            }
            if graph.get_node(&to).is_none() {
                eprintln!(" Target node '{}' not found", to);
                return;
            }

            graph.add_edge(from.clone(), to.clone());

            if let Err(e) = graph.save_to_file("graph.json") {
                eprintln!(" Failed to save: {}", e);
                return;
            }
            println!("Edge created: {} → {}", from, to);
        }

        Commands::ListNodes => {
            let nodes = graph.list_nodes();

            if nodes.is_empty() {
                println!("  No nodes found.");
            } else {
                println!(" Nodes ({}):", nodes.len());
                println!("{:-<60}", "");
                for node in nodes {
                    println!("  {} - {}", node.id, node.title);
                    if !node.content.is_empty() {
                        println!("    {}", node.content);
                    }
                    if !node.tags.is_empty() {
                        println!("    Tags: {}", node.tags.join(", "));
                    }
                    println!();
                }
            }
        }

        Commands::Search { query } => {
            let results = graph.search_filter(query.clone());

            if results.is_empty() {
                println!(" No match found for: '{}'", query);
            } else {
                println!(" Found {} result(s) for '{}':", results.len(), query);
                println!("{:-<60}", "");
                for node in results {
                    println!("  {} - {}", node.id, node.title);
                    if !node.content.is_empty() {
                        println!("    {}", node.content);
                    }
                    if !node.tags.is_empty() {
                        println!("    Tags: {}", node.tags.join(", "));
                    }
                    println!();
                }
            }
        }

        Commands::Tui => {
            println!(" Launching TUI (press q to quit)...");
            if let Err(e) = tui::launch_tui(&graph) {
                eprintln!(" TUI error: {}", e);
            }
        }
    }
}
