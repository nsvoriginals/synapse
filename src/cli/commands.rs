use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "graph-cli", version, about = "Graph knowledge manager CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new node
    AddNode {
        /// Optional custom ID (generates UUID if not provided)
        #[arg(short, long)]
        id: Option<String>,

        /// Title of the node
        #[arg(short, long)]
        title: String,

        /// Content description
        #[arg(short, long, default_value = "")]
        content: String,

        /// Tags for categorization
        #[arg(short = 'T', long = "tag")]
        tags: Vec<String>,
    },

    /// Add an edge between nodes
    AddEdge {
        /// Source node ID
        from: String,

        /// Target node ID
        to: String,
    },

    /// List all nodes
    ListNodes,

    /// Search nodes by title, content, or tags
    Search {
        /// Search query string
        query: String,
    },

    /// Open the TUI
    Tui,
}
