# promptctl 🤖

An interactive CLI tool for generating and improving AI prompts using Claude. This tool helps you create more effective prompts through guided refinement and best practices.

## Features

- 🔄 **Interactive Prompt Generation**: Create better prompts through guided interaction with Claude
- 🎯 **Follow-up Questions**: Use the `-r` flag to enable follow-up questions that make your prompts more specific and effective
- 📋 **Clipboard Integration**: Improved prompts are automatically copied to your clipboard
- 🔗 **Direct AI Integration**: Open your improved prompts directly in Claude or ChatGPT
- 🎨 **Beautiful Terminal UI**: Clean and intuitive interface with progress indicators
- 🌐 **Web-based Architecture**: No API key management needed - uses a secure web API

## Installation

```bash
# to be updated
```

## Usage

### Generate Improved Prompts

Basic prompt generation:

```bash
pctl generate  # full command
pctl g         # shorthand
```

Enable follow-up questions for more refined prompts:

```bash
pctl generate --refine  # full command
pctl g -r               # shorthand
```

### Example Workflow

1. Start with a basic prompt idea:

   ```bash
   $ pctl g
   Enter your prompt: write a blog post about rust
   ```

2. Or use refinement mode for more specific prompts:

   ```bash
   $ pctl g -r
   Enter your prompt: write a blog post about rust
   What specific aspect of Rust would you like to focus on? memory safety
   Who is your target audience? beginner programmers
   ```

3. The tool will generate an improved prompt and:
   - Display it in a formatted box
   - Copy it to your clipboard
   - Offer options to open in Claude or ChatGPT

## Project Status

This project is part of a phased development approach:

✅ **Phase 1 (Current)**: MVP

- Basic CLI with interactive prompt generation
- Follow-up questions for refinement
- Clean prompt formatting
- Direct AI integration

🔜 **Phase 2**: Enhanced Features

- [ ] Template support
- [ ] History management
- [ ] Improved prompt analysis
- [ ] Multiple AI model integration

## Contributing

Contributions are welcome! Please check our [Contributing Guide](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
