# 🧠 Synapse

> A personal knowledge graph for your brain dumps, built in Rust.

Synapse is a command-line tool that helps you capture thoughts, ideas, and notes while preserving the connections between them. Think of it as your own personal Wikipedia with backlinks, timelines, and a beautiful terminal UI.

## ✨ Features

- **📝 Quick Note Taking** - Capture ideas instantly from the command line
- **🔗 Link Everything** - Create bidirectional links between related notes
- **🏷️ Smart Tagging** - Organize with tags and filter by them
- **🌳 Tree View** - Visualize your knowledge graph in an interactive TUI
- **🔍 Fast Search** - Find notes by title, content, or tags
- **⏰ Timeline Queries** - See what you were thinking about last week, month, or year
- **💾 JSON Storage** - Simple, portable file format for your graph
- **🎨 Beautiful Terminal UI** - Navigate your thoughts with an intuitive interface

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/synapse.git
cd synapse

# Build the project
cargo build --release

# Optional: Install globally
cargo install --path .
```

### Basic Usage

```bash
# Create your first note
synapse add "Rust Ownership" "The ownership system prevents data races"

# Add tags to a note
synapse add "Learning Goals" "Master Rust by 2026" --tags goals,programming

# List all notes
synapse list

# Search for notes
synapse search "rust"

# View a specific note
synapse view <note-id>

# Link two notes together
synapse link <from-id> <to-id>

# Launch the interactive TUI
synapse tui
```

## 📖 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `add` | Create a new note | `synapse add "Title" "Content"` |
| `list` | List all notes | `synapse list` |
| `view` | View a specific note | `synapse view abc123` |
| `edit` | Edit a note's content | `synapse edit abc123` |
| `delete` | Delete a note | `synapse delete abc123` |
| `search` | Search notes by keyword | `synapse search ownership` |
| `link` | Link two notes | `synapse link abc123 def456` |
| `tag` | Add tags to a note | `synapse tag abc123 rust,learning` |
| `tui` | Launch interactive tree view | `synapse tui` |
| `export` | Export to Markdown/DOT | `synapse export --format md` |

## 🎮 TUI Controls

When in the interactive tree view (`synapse tui`):

- **↑/↓** or **j/k** - Navigate up/down
- **Enter** or **→** - Expand node
- **←** - Collapse node  
- **q** or **Esc** - Quit
- **/** - Search mode *(coming soon)*
- **a** - Quick actions *(coming soon)*

## 🏗️ Project Structure

```
synapse/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── models/          # Data structures
│   │   ├── node.rs      # Note representation
│   │   ├── edge.rs      # Link representation
│   │   └── graph.rs     # Graph container
│   ├── storage/         # Persistence layer
│   ├── cli/             # Command handlers
│   └── tui/             # Terminal UI
└── data/
    └── synapse.json     # Your knowledge graph
```

## 💡 Example Workflow

```bash
# Capture an idea
synapse add "Rust Async" "Need to learn about async/await in Rust"

# Later, add related concepts
synapse add "Tokio Runtime" "Most popular async runtime for Rust"
synapse add "Futures" "Core abstraction for async in Rust"

# Connect them
synapse link <rust-async-id> <tokio-id>
synapse link <rust-async-id> <futures-id>

# Add context with tags
synapse tag <rust-async-id> learning,rust,async

# Visualize the connections
synapse tui
```

## 🛠️ Built With

- **[clap](https://github.com/clap-rs/clap)** - Command-line argument parsing
- **[serde](https://github.com/serde-rs/serde)** - Serialization framework
- **[ratatui](https://github.com/ratatui-org/ratatui)** - Terminal UI framework
- **[uuid](https://github.com/uuid-rs/uuid)** - Unique ID generation
- **[chrono](https://github.com/chronotope/chrono)** - Date and time handling

## 🎯 Roadmap

- [x] Basic note creation and storage
- [x] Bidirectional linking
- [x] Tag system
- [x] Interactive TUI
- [ ] Full-text search
- [ ] Fuzzy search with highlighting
- [ ] In-TUI note editing
- [ ] Export to Markdown/HTML
- [ ] Graph visualization (DOT format)
- [ ] Collision detection for similar notes
- [ ] Random walk through connections
- [ ] Statistics dashboard

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest features
- Submit pull requests

## 📄 License

MIT License - feel free to use this for your own brain dumps!

## 💭 Philosophy

Your thoughts aren't linear—why should your notes be? Synapse embraces the interconnected nature of ideas, helping you build a second brain that grows organically over time.

---

**Built with ❤️ and Rust** 🦀