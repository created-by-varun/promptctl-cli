# promptctl 🤖

A CLI tool for generating and improving AI prompts using Claude. This tool helps you create more effective prompts through interactive refinement and best practices.

## Features

- 🔄 **Interactive Prompt Refinement**: Use the `--refine` flag to get helpful follow-up questions that make your prompts more specific and effective
- 📋 **Clipboard Integration**: Improved prompts are automatically copied to your clipboard
- 🔗 **Direct AI Integration**: Open your improved prompts directly in Claude or ChatGPT
- 🎨 **Beautiful Terminal UI**: Clean and intuitive interface with progress indicators
- 🔑 **Simple Configuration**: Easy API key management

## Installation

1. Ensure you have Rust installed ([Install Rust](https://rustup.rs/))
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/promptctl.git
   cd promptctl
   ```
3. Build and install:
   ```bash
   cargo install --path .
   ```

## Usage

### Configure Claude API Key

Before using the tool, set up your Claude API key (use any of these):

```bash
pctl config --claude-key "your-api-key-here"  # full command
pctl config -k "your-api-key-here"           # shorthand flag
pctl c --claude-key "your-api-key-here"      # shorthand command
pctl c -k "your-api-key-here"               # all shorthands
```

View your current configuration:

```bash
pctl config --view  # full command
pctl c --view       # shorthand
```

### Generate Improved Prompts

Generate and refine a prompt (use either command):

```bash
pctl generate  # full command
pctl g         # shorthand
```

Enable follow-up questions for better prompts (use any of these):

```bash
pctl generate --refine  # full command
pctl generate -r        # shorthand flag
pctl g --refine         # shorthand command
pctl g -r               # all shorthands
```

## Dependencies

- [clap](https://crates.io/crates/clap): Command-line argument parsing
- [dialoguer](https://crates.io/crates/dialoguer): Interactive terminal prompts
- [reqwest](https://crates.io/crates/reqwest): HTTP client for API communication
- [serde](https://crates.io/crates/serde): JSON serialization
- [tokio](https://crates.io/crates/tokio): Async runtime
- [colored](https://crates.io/crates/colored): Terminal text styling

## Contributing

Contributions are welcome! The project is currently in its MVP phase, focusing on:

1. Basic CLI functionality
2. Interactive prompt refinement
3. Clean prompt formatting
4. Direct AI integration

Future phases will include:

- Template support
- History management
- Multiple AI model integration
- Collaborative features

## License

This project is licensed under the MIT License - see the LICENSE file for details.
