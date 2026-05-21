# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**ace-ctx** — 基于 Rust 的 CLI 工具 + MCP 服务器，提供代码库索引、语义搜索和提示增强功能。以 npm 包形式分发（`npx @alistar.max/ace-ctx`），通过 GitHub Releases 提供预构建二进制文件（6 平台），Node.js wrapper (run.js) 实现自动下载。

## Build & Test Commands

```bash
# 编译
cargo check --all-targets --all-features

# 配置环境变量简化命令（替代 --base-url / --token）
export ACE_BASE_URL="https://api.example.com"
export ACE_TOKEN="your-token"

# 4 种运行模式
cargo run                                          # MCP 服务器模式（默认）
cargo run -- --search "如何实现用户认证"              # CLI 语义搜索（stdout 输出）
cargo run -- --enhance-prompt "重构认证模块"          # CLI 提示增强（stdout 输出）
cargo run -- --index-only                           # 仅增量索引，退出

# CLI 模式常用参数
cargo run -- --search "query" --include-document-files   # 搜索包含文档文件
cargo run -- --enhance-prompt "prompt" --no-webbrowser   # 增强跳过浏览器编辑
cargo run -- --transport lsp                              # MCP LSP 传输模式

# 测试
cargo test --lib                           # 单元测试（390+）
cargo test --test '<name>'                 # 集成测试（如 cargo test --test 'index_test'）
cargo test --lib -- <test_name>            # 单个单元测试
cargo test --test 'index_test' -- <name>   # 单个集成测试
cargo test --doc                           # 文档测试

# 格式化与 lint
cargo fmt --all -- --check                 # 格式检查
cargo fmt --all                            # 自动格式化
cargo clippy --all-targets --all-features -- -D warnings

# 安全审计
cargo audit
```

## Project Architecture

### 顶层结构

```
src/
├── main.rs               # CLI 入口，clap 参数解析 + 4 模式调度
│                          # 调度顺序: search → enhance_prompt → index_only → MCP server
│                          # 支持 ACE_BASE_URL / ACE_TOKEN 环境变量回退
├── lib.rs                # 库入口，重新导出 Config, IndexManager, PromptEnhancer 等
├── config.rs             # 配置管理（base_url, token, 默认的 text_extensions, exclude_patterns）
├── search_filter.rs      # 搜索过滤：扩展名/文件名/glob 排除，支持"源码优先"模式
├── http_logger.rs        # HTTP 请求/响应日志记录
│
├── index/
│   ├── mod.rs
│   └── manager.rs        # 核心索引引擎
│     ├── 文件扫描: WalkDir + .gitignore/.aceignore 感知
│     ├── 编码检测: UTF-8/GBK/GB18030/Windows-1252
│     ├── SHA-256 内容指纹 + mtime 增量索引
│     ├── 语义搜索: 上传 query blob → 远程向量检索
│     ├── 自适应批量上传: FuturesUnordered + AIMD 速率控制
│     └── 持久化: .ace-tool/index.json (bincode)
│
├── mcp/
│   ├── server.rs          # MCP 服务器（JSON-RPC 2.0）
│   │ ├── 传输协议: LSP (Content-Length) / Line / Auto
│   │ ├── 方法: initialize, tools/list, tools/call, ping
│   │ └── 工具: search_context, enhance_prompt（可禁用）
│   └── types.rs          # MCP/JSON-RPC 类型定义
│
├── tools/
│   ├── mod.rs
│   ├── search_context.rs  # MCP 工具: search_context 实现
│   └── enhance_prompt.rs  # MCP 工具: enhance_prompt 实现
│
├── enhancer/
│   ├── mod.rs
│   ├── prompt_enhancer.rs # 提示增强引擎（new/old/claude/openai/gemini/codex 6 端点）
│   ├── server.rs          # Web UI HTTP 服务器（hyper）
│   └── templates.rs       # HTML/提示模板
│
├── service/
│   ├── common.rs          # EnhancerEndpoint, ThirdPartyConfig 共享类型
│   ├── augment.rs         # Augment API（new + old 端点）
│   ├── claude.rs          # Anthropic Claude API
│   ├── openai.rs          # OpenAI API
│   ├── gemini.rs          # Google Gemini API
│   └── codex.rs           # OpenAI Codex (Responses API)
│
├── strategy/
│   ├── adaptive.rs        # AIMD 自适应上传策略（TCP 拥塞控制启发）
│   │ ├── 暖启动 → 加性增/乘性减
│   │ └── 指标: 成功率(滑动窗口)、EWMA 延迟、速率限制感知
│   └── metrics.rs         # 运行时指标（滑动窗口 20 + EWMA α=0.2）
│
└── utils/
    ├── path_normalizer.rs # 跨平台路径规范（WSL UNC、/mnt/ 转换）
    └── project_detector.rs # .ace-tool 目录管理、自动 .gitignore 添加
```

### 关键环境变量

| 变量 | 用途 |
|------|------|
| `ACE_BASE_URL` | API base URL（替代 `--base-url` CLI 参数） |
| `ACE_TOKEN` | API token（替代 `--token` CLI 参数） |
| `PROMPT_ENHANCER_ENDPOINT` | 增强端点: new(old) / claude / openai / gemini / codex |
| `PROMPT_ENHANCER_BASE_URL` | 第三方 API base URL（claude/openai/gemini/codex 时使用） |
| `PROMPT_ENHANCER_TOKEN` | 第三方 API token |
| `PROMPT_ENHANCER_MODEL` | 第三方 API 模型名 |
| `PROMPT_ENHANCER_INCLUDE_SEARCH_CONTEXT` | 设为 "1"/"true" 为第三方端点注入 search_context |
| `PROMPT_ENHANCER` | 设为 "disabled" 禁用 enhance_prompt 工具 |
| `ACE_ENHANCER_ENDPOINT` | 旧版端点配置（PROMPT_ENHANCER_ENDPOINT 的向后兼容回退） |
| `RUST_LOG` | 日志级别（info / debug / warn） |

增强端点默认模型: Claude → `claude-sonnet-4-5`, OpenAI → `gpt-5.2`, Gemini → `gemini-3-flash-preview`, Codex → `gpt-5.3-codex`

### 数据流

1. **索引流程**: `WalkDir 扫描 → .gitignore 过滤 → 编码检测 → SHA-256 指纹 → mtime 增量对比 → 批量上传(AIMD自适应) → 持久化 .ace-tool/index.json`

2. **搜索流程**: `自然语言查询 → 上传 query blob → 远程向量检索 → SearchFilterOptions 过滤排除文件 → 返回代码片段`

3. **增强流程**: `原始提示 + 对话历史 + [可选: search_context 注入] → 调用 API → [可选: Web UI 编辑确认] → 返回增强后提示`

4. **CLI MCP 服务器模式**: `main.rs 4 模式调度 → 默认: MCP server → stdio (LSP/Line/Auto 传输) → JSON-RPC 2.0 协议`

### 关键设计模式

- **4 模式入口调度**: `main.rs` 按 `--search` → `--enhance-prompt` → `--index-only` → 默认 MCP server 顺序检查，每种模式完成后直接 exit，不相互承载
- **搜索源码优先**: `SearchFilterOptions` 默认排除 `.md/.txt/README/CHANGELOG` 等文档文件，CLI 的 `--include-document-files` 可覆盖
- **AIMD 自适应**: 策略层实现 TCP 拥塞控制启发的上传优化，并发 1~8，超时 15s~180s，可通过 `--upload-concurrency` / `--upload-timeout` 覆盖
- **Config 共享**: 所有组件共享 `Arc<Config>`，Config 内部硬编码 100+ 种 text_extensions 和 80+ exclude_patterns
- **第三方增强端点**: 使用 `PROMPT_ENHANCER_*` 环境变量，`--enhance-prompt` 模式下 `--base-url`/`--token` 成为可选项

### AI Agent Skill 集成

仓库提供 `skills/ace-code-search-expert/SKILL.md`，为 Claude Code 等 AI Agent 提供"预上下文窄管道"：
- Agent 识别用户代码问题意图 → 后台调用 `ace-ctx --search` → 获取高精度代码片段 → 节省 Token 消耗
- 需要全局安装 ace-ctx 并配置 `ACE_BASE_URL` / `ACE_TOKEN`

### 测试策略

- 单元测试位于各模块末尾 `#[cfg(test)] mod tests` 中（390+）
- 集成测试位于 `tests/` 目录（每个文件对应一个功能模块: index_test, mcp_test, prompt_enhancer_test 等）
- 使用 `tempfile` + `wiremock` 进行文件 IO 和 HTTP 模拟测试
- 涉及环境变量的测试使用 `Mutex` 保持串行执行

### 发布与分布

- npm 包 `@alistar.max/ace-ctx` 通过 GitHub Releases 预构建二进制发布
- `npm/run.js` Node.js wrapper 负责: 检测平台 → 按版本缓存 → 从 GitHub Releases 下载 → spawn 执行二进制
- 平台特定可选依赖: `darwin-universal`, `linux-x64`, `linux-arm64`, `win32-x64`, `win32-arm64`
- 商业使用需要 Commercial License（联系 missdeer@gmail.com）
