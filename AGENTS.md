# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**ace-tool-rs** — 基于 Rust 的 MCP 服务器，提供代码库索引、语义搜索和提示增强功能。以 npm 包形式分发，通过平台特定的可选依赖提供预构建二进制文件。

## Build & Test Commands

```bash
# 编译
cargo check --all-targets --all-features

# 运行
cargo run -- --base-url <URL> --token <TOKEN>

# 运行模式
cargo run -- --search "query" --base-url <URL> --token <TOKEN>    # 语义搜索模式
cargo run -- --enhance-prompt "prompt"    --base-url <URL> --token <TOKEN>  # 增强提示模式
cargo run -- --index-only --base-url <URL> --token <TOKEN>       # 仅索引模式

# 测试
cargo test --lib                           # 单元测试
cargo test --test '<name>'                 # 集成测试（如: cargo test --test 'index_test'）
cargo test --lib -- <test_name>            # 单个单元测试
cargo test --test 'prompt_enhancer_test' -- test_name  # 单个集成测试
cargo test --doc                           # 文档测试
cargo test --all-features                  # 全部特性全部测试

# 格式化与lint
cargo fmt --all -- --check                 # 格式检查
cargo fmt --all                            # 自动格式化
cargo clippy --all-targets --all-features -- -D warnings  # 静态检查

# 安全审计
cargo audit
```

## Project Architecture

### 顶层结构

```
src/
├── main.rs               # CLI 入口，clap 参数解析 + 多模式调度
│                          # 支持: MCP server, search, enhance, index-only 四种模式
├── lib.rs                # 库入口，重新导出常用类型
├── config.rs             # 配置管理（base_url, token, text_extensions, exclude_patterns）
├── search_filter.rs      # 动态文档排除（扩展名/文件名/glob 过滤）
│
├── index/
│   ├── mod.rs
│   └── manager.rs        # 核心索引引擎（1770行）
│     ├── 文件扫描 WalkDir + .gitignore 感知
│     ├── 编码检测 (UTF-8/GBK/GB18030/Windows-1252)
│     ├── SHA-256 内容指纹 + 增量索引
│     ├── 语义搜索（上传 query blob → 远程检索）
│     └── 自适应批量上传（FuturesUnordered + 速率控制）
│
├── mcp/
│   ├── mod.rs
│   ├── server.rs          # MCP 服务器（JSON-RPC 2.0）
│   │ ├── 传输协议: LSP (Content-Length header) / Line (JSON per line) / Auto
│   │ ├── MCP 方法: initialize, tools/list, tools/call, ping
│   │ └── 工具: search_context, enhance_prompt（可禁用）
│   └── types.rs          # MCP/JSON-RPC 数据结构
│
├── enhancer/
│   ├── mod.rs
│   ├── prompt_enhancer.rs # 提示增强引擎
│   │ ├── 支持多个 API 端: new, old, claude, openai, gemini, codex
│   │ ├── Web UI 交互（浏览器编辑/确认增强结果）
│   │ └── 可选的 search_context 注入（PROMPT_ENHANCER_INCLUDE_SEARCH_CONTEXT）
│   ├── server.rs         # Web UI HTTP 服务器（hyper 实现）
│   └── templates.rs      # HTML/提示模板
│
├── service/
│   ├── mod.rs
│   ├── common.rs         # 共享类型: EnhancerEndpoint, ThirdPartyConfig
│   ├── augment.rs        # Augment API 调用（新增/new + 旧版/old 端点）
│   ├── claude.rs         # Anthropic Claude API
│   ├── openai.rs         # OpenAI API
│   ├── gemini.rs         # Google Gemini API
│   └── codex.rs          # OpenAI Codex (Responses API)
│
├── strategy/
│   ├── mod.rs
│   ├── adaptive.rs       # AIMD 自适应上传策略
│   │ ├── 暖启动 → 加性增/乘性减
│   │ └── 指标: 成功率、EWMA 延迟、速率限制感知
│   └── metrics.rs        # 运行时指标收集（滑动窗口 + EWMA）
│
└── utils/
    ├── mod.rs
    ├── path_normalizer.rs # 跨平台路径规范化（WSL UNC、/mnt/ 转换）
    └── project_detector.rs # .ace-tool 目录管理、自动添加 .gitignore
```

### 关键环境变量

| 变量 | 用途 |
|------|------|
| `PROMPT_ENHANCER_ENDPOINT` | 增强端点: new / old / claude / openai / gemini / codex |
| `PROMPT_ENHANCER_BASE_URL` | 第三方 API base URL（claude/openai/gemini 时使用） |
| `PROMPT_ENHANCER_TOKEN` | 第三方 API token |
| `PROMPT_ENHANCER_MODEL` | 第三方 API 模型名 |
| `PROMPT_ENHANCER_INCLUDE_SEARCH_CONTEXT` | 设为 "1"/"true" 为第三方端点注入 search_context |
| `PROMPT_ENHANCER` | 设为 "disabled" 禁用 enhance_prompt 工具 |
| `ACE_ENHANCER_ENDPOINT` | 旧版端点配置（向后兼容） |

### 数据流

1. **索引流程**：`WalkDir 扫描 → .gitignore 过滤 → 编码检测 → SHA-256 指纹 → 增量对比 → 批量上传（自适应） → 持久化索引 JSON 到 .ace-tool/`

2. **搜索流程**：`自然语言查询 → 上传 query blob → 远程向量检索 → 过滤排除文件 → 返回代码片段`

3. **增强流程**：`原始提示 + 对话历史 + [可选: search_context 注入] → 调用 API → [Web UI 编辑] → 返回增强后提示`

### 测试策略

- 单元测试位于各模块末尾的 `#[cfg(test)] mod tests` 中
- 集成测试位于 `tests/` 目录（每个文件对应一个模块）
- 使用 `tempfile` + `wiremock` 进行文件 IO 和 HTTP 模拟测试
- 涉及环境变量的测试使用 `Mutex` 保持串行执行

### 发布与分布

- 通过 `cargo-release` 自动发布到 npm（`npm/` 目录）
- 平台二进制通过 `napi-rs` 构建（`scripts/build-macOS-universal-binary.sh`）
- 支持 6 个平台: darwin-universal, linux-x64, linux-arm64, win32-x64, win32-arm64
