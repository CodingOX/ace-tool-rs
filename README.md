# ace-ctx

English | [简体中文](README-zh-CN.md)

A high-performance CLI codebase context engine written in Rust, designed for AI Agents and developers, supporting lightning-fast, low-overhead semantic search in terminal driven by **Skill packages**.

> [!IMPORTANT]
> **Special Thanks**: This project is a fork of the original repository [missdeer/ace-tool-rs](https://github.com/missdeer/ace-tool-rs). We would like to express our deepest gratitude to the original author `missdeer` for his outstanding contribution and open-source spirit! This project adapts and expands upon his work specifically for AI Agent scenarios (particularly driven by Skills).

---

## 🌟 Core Highlight: AI Agent Skill Integration

If you are using next-generation agentic coding frameworks or IDEs that support custom `Skills` (such as **Claude Code**, **Antigravity IDE**, etc.), `ace-ctx` provides a highly optimized, ready-to-use AI Agent Skill out of the box!

### 1. Skill Location
The skill definition is stored in the repository at:
👉 [skills/ace-code-search-expert/SKILL.md](skills/ace-code-search-expert/SKILL.md)

### 2. How It Works & Advantages (Pre-context Narrow Pipe)
Once the agent imports this Skill, it will automatically detect your code search intent and silently invoke the global `ace-ctx --search` CLI tool in the background.
- **Lightning-fast Search**: Performs vector matching directly against local and remote indexing services instead of the agent blindly browsing files.
- **Save Massive Tokens**: The agent only feeds the highly reliable code snippets back as a condensed context, saving a massive amount of token consumption and context window space.
- **Eliminate Hallucinations**: Precise line numbers and file location mappings eliminate model hallucinations caused by lack of localized context.

### 3. How to Import It for Your AI Agent
1. **Set Environment Variables**: Configure system environment variables `ACE_BASE_URL` and `ACE_TOKEN`.
2. **Import the Skill**: Copy the absolute path of [skills/ace-code-search-expert/SKILL.md](skills/ace-code-search-expert/SKILL.md) and import it into your agent workspace's skill definition directory.
   > [!NOTE]
   > The agent automatically performs environment sniffing when executing this skill: if `ace-ctx` is not globally installed on your system, it will automatically run zero-install via `npx -y`, eliminating the need for manual global installation. However, for maximum execution speed, you can still choose to install it globally via `npm install -g @alistar.max/ace-ctx`.

---

## 💻 Standalone Codebase Search Mode (CLI Search Mode)

Once your environment variables are configured, you can perform quick natural language semantic search directly in your terminal:

```bash
# Semantic search against the codebase (defaults to source code first, ignoring document noise)
ace-ctx --search "How does user login connect to the database?"

# If you explicitly want to search including document files (like .md, .txt, etc.)
ace-ctx --search "How to deploy and install this project" --include-document-files
```

The search results print highly relevant code snippets, complete with syntax highlighting and line numbers, directly to stdout—perfect for terminal reading and AI agent parsing.

---

## 🔧 Easy Setup & Installation

### 1. Quick Start (Recommended)
The easiest way to install and run `ace-ctx` is via npx:
```bash
npx @alistar.max/ace-ctx --base-url <API_URL> --token <AUTH_TOKEN>
```
This automatically downloads the appropriate binary for your platform (supporting Windows, macOS, and Linux).

### 2. Environment Variables Quick Config
To avoid typing verbose CLI parameters on every run, configure them as system environment variables:
```bash
# macOS/Linux (Zsh/Bash)
export ACE_BASE_URL="https://api.example.com"
export ACE_TOKEN="your-token-here"

# Run it simply and elegantly:
ace-ctx --search "find user module"
```

### 3. From Source
```bash
git clone https://github.com/CodingOX/ace-ctx.git
cd ace-ctx
cargo build --release
# The binary will be at target/release/ace-ctx
```

---

## ⚡ Other Standalone Modes

Besides the core **CLI Search Mode**, `ace-ctx` also supports the following auxiliary capabilities:

1. **Incremental Indexing Mode (Index-Only Mode)**
   Scan the project directory, upload incremental index vectors, and exit immediately upon completion. Perfect for Git Commit Hooks or CI pipelines.
   ```bash
   ace-ctx --index-only
   ```
2. **In-Terminal Prompt Enhancement Mode (Enhance-Prompt Mode)**
   Quickly rewrite, optimize, and enrich your prompt with local codebase context right in the terminal.
   ```bash
   ace-ctx --enhance-prompt "Refactor user authentication module"
   ```
3. **MCP Server Mode (Model Context Protocol)**
   Runs as a standard background MCP server over stdio to expose `search_context` and `enhance_prompt` tools for MCP-compatible clients.
   ```bash
   ace-ctx --base-url <API_URL> --token <AUTH_TOKEN>
   ```

---

## 📄 License

This project is dual-licensed:
- **Non-Commercial / Personal Use**: Licensed under the [GNU General Public License v3.0](LICENSE).
- **Commercial / Workplace Use**: If you use it in a commercial environment, workplace, or for any commercial purpose, you must obtain a commercial license. Contact `missdeer@gmail.com` for licensing inquiries.

---

## 👥 Contributing & Support

Contributions are welcome! Please feel free to submit a Pull Request.
- Original author and contributor: [missdeer](https://github.com/missdeer)
- Thanks for the support and feedback from the friends at [LINUX DO](https://linux.do/).

[![Star History Chart](https://starchart.cc/CodingOX/ace-ctx.svg)](https://starchart.cc/CodingOX/ace-ctx)

