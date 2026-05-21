# ACE-Ctx 编译部署指南

## 环境要求

- Rust 1.70+ (推荐使用 `rustup` 安装)
- macOS / Linux

## 编译步骤

### 1. 克隆项目

```bash
git clone https://github.com/CodingOX/ace-ctx.git
cd ace-ctx
```

### 2. 编译 Release 版本

```bash
cargo build --release
```

编译产物位于：`./target/release/ace-ctx`

### 3. 运行测试（可选）

```bash
cargo test --test index_test --test tools_test
cargo test --lib
```

## 部署方式

### 方式一：用户目录（推荐，无需 sudo）

```bash
mkdir -p ~/.local/bin
install -m 755 ./target/release/ace-ctx ~/.local/bin/

# 添加到 PATH（如需要）
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 方式二：系统目录

```bash
sudo install -m 755 ./target/release/ace-ctx /usr/local/bin/
```

## 验证安装

```bash
ace-ctx --help
```

输出应显示：
```
CLI tool and MCP server for codebase indexing and semantic search

Usage: ace-ctx [OPTIONS]

Options:
      --base-url <BASE_URL>              API base URL for the indexing service [env: ACE_BASE_URL]
      --token <TOKEN>                    Authentication token [env: ACE_TOKEN]
      --transport <TRANSPORT>            Transport framing: auto, lsp, line [default: auto]
      --max-lines-per-blob <N>           Maximum lines per blob (default: 800)
      --upload-timeout <SECONDS>         Upload timeout in seconds (default: adaptive)
      --upload-concurrency <N>           Upload concurrency (default: adaptive)
      --retrieval-timeout <SECONDS>      Retrieval timeout in seconds (default: 60)
      --no-adaptive                      Disable adaptive strategy
      --no-webbrowser-enhance-prompt     Disable web browser for enhance_prompt
      --force-xdg-open                   Force xdg-open in WSL
      --webui-addr <ADDR:PORT>           Web UI bind address (e.g., "127.0.0.1:8754")
      --index-only                       Index current directory and exit
      --enhance-prompt <PROMPT>          Enhance a prompt and output to stdout
      --search <QUERY>                   Search codebase with natural language query
      --include-document-files           Include document files in search results
  -V, --version                          Print version
  -h, --help                             Show help
```

## CLI 使用示例

ace-ctx 支持四种运行模式，可通过不同的命令行参数切换：

| 模式 | 触发参数 | 说明 |
|------|----------|------|
| MCP Server（默认） | 无特殊参数 | 启动 MCP 服务器，通过 stdio 与 MCP 客户端通信 |
| 语义搜索 | `--search` | 搜索代码库并输出结果到 stdout，然后退出 |
| 提示增强 | `--enhance-prompt` | 增强提示并输出结果到 stdout，然后退出 |
| 仅索引 | `--index-only` | 索引当前目录后退出，不启动服务 |

### 配置凭证

`--base-url` 和 `--token` 支持两种配置方式，CLI 参数优先级高于环境变量：

```bash
# 方式一：通过环境变量（推荐，避免每次手动输入）
export ACE_BASE_URL="https://your-api-server.com"
export ACE_TOKEN="your-auth-token"

# 方式二：通过 CLI 参数（优先级更高，会覆盖环境变量）
ace-ctx --base-url https://your-api-server.com --token your-token
```

> **提示**：推荐将环境变量写入 `~/.zshrc` 或 `~/.bashrc`，这样后续所有命令都无需重复指定。

### 模式一：MCP Server（默认模式）

启动 MCP 服务器，通过 stdio 协议与 AI 客户端（如 Claude Desktop）通信：

```bash
# 基本启动（环境变量已配置时）
ace-ctx

# 显式指定凭证
ace-ctx --base-url https://your-api-server.com --token your-token

# 指定传输协议（auto/lsp/line）
ace-ctx --transport lsp
```

### 模式二：语义搜索

使用自然语言查询搜索代码库，结果输出到 stdout：

```bash
# 基本搜索（需要先 cd 到项目目录）
cd /path/to/your-project
ace-ctx --search "用户认证流程是怎么实现的"

# 搜索并包含文档类文件（.md, .txt 等）
ace-ctx --search "API 错误处理逻辑" --include-document-files

# 搜索结果可以配合管道使用
ace-ctx --search "数据库连接池配置" | head -50
```

### 模式三：提示增强

增强自然语言提示，注入项目上下文信息：

```bash
# 使用默认端点增强提示
cd /path/to/your-project
ace-ctx --enhance-prompt "帮我分析这个项目的架构设计"

# 禁用 Web UI 浏览器交互，直接输出结果
ace-ctx --enhance-prompt "重构用户模块" --no-webbrowser-enhance-prompt
```

使用第三方 LLM 端点（claude/openai/gemini/codex）进行增强时，通过环境变量配置：

```bash
# 使用 OpenAI 端点
export PROMPT_ENHANCER_ENDPOINT="openai"
export PROMPT_ENHANCER_BASE_URL="https://api.openai.com/v1"
export PROMPT_ENHANCER_TOKEN="sk-xxx"
export PROMPT_ENHANCER_MODEL="gpt-4o"

ace-ctx --enhance-prompt "优化这段代码的性能"

# 使用第三方端点时，base-url/token 是可选的（仅用于启用 search_context 注入）
# 不提供则仅依赖第三方 LLM 自身能力
```

### 模式四：仅索引

只索引当前目录下的文件，不启动 MCP 服务。适合 CI/CD 预热或定时索引任务：

```bash
cd /path/to/your-project
ace-ctx --index-only
```

### 高级参数

```bash
# 调整索引性能参数
ace-ctx --index-only \
  --max-lines-per-blob 1000 \
  --upload-timeout 120 \
  --upload-concurrency 4

# 禁用自适应上传策略（使用固定参数）
ace-ctx --index-only --no-adaptive

# 增加搜索检索超时时间（大型项目）
ace-ctx --search "复杂查询" --retrieval-timeout 120

# 指定 Web UI 绑定地址（enhance_prompt 模式）
ace-ctx --enhance-prompt "优化代码" --webui-addr "0.0.0.0:8754"

# WSL 环境下强制使用 xdg-open 打开浏览器
ace-ctx --enhance-prompt "分析项目" --force-xdg-open
```

## 版本号

版本号定义在 `Cargo.toml` 的 `version` 字段：

```toml
[package]
name = "ace-ctx"
version = "0.2.4"
```

### 升级版本

1. 修改 `Cargo.toml` 中的 `version` 字段
2. 重新编译：`cargo build --release`
3. 重新部署

## 运行 MCP 服务

### 环境变量配置

```bash
export ACE_BASE_URL="https://your-api-server.com"
export ACE_TOKEN="your-auth-token"
```

### 命令行启动

```bash
ace-ctx --base-url https://your-api-server.com --token your-token
```

### 作为 MCP 服务使用

在 Claude Desktop 或其他 MCP 客户端配置：

```json
{
  "mcpServers": {
    "ace-ctx": {
      "command": "/path/to/ace-ctx",
      "args": [],
      "env": {
        "ACE_BASE_URL": "https://your-api-server.com",
        "ACE_TOKEN": "your-auth-token"
      }
    }
  }
}
```

## MCP 模式中的动态过滤

通过 MCP 模式（如 Claude Code 或 Cursor 集成）运行时，过滤参数在每次 `search_context` 工具调用时通过 arguments 传递：

```json
{
  "project_root_path": "/path/to/project",
  "query": "用户认证流程",
  "exclude_document_files": true,
  "exclude_extensions": [".txt", ".csv"],
  "exclude_globs": ["**/temp/**", "docs/**"]
}
```

### 过滤规则说明

- `exclude_document_files`: 快捷开关，排除 `.md`, `.mdx`, `.txt`, `.csv`, `.tsv`, `.rst`, `.adoc`, `.tex`, `.org` 等文档类文件
- `exclude_extensions`: 精确按扩展名排除（带前导 `.`，大小写不敏感）
- `exclude_globs`: 按路径模式排除，使用标准 glob 语法
- 三者关系为 **Union（并集）**，满足任一条件的文件即被排除

## CI (持续集成) 使用与配置说明

将 `ace-ctx` 集成到 CI（例如 GitHub Actions、GitLab CI）中，可以在代码每次推送或提 PR 时，**自动对代码库进行合规性审查、敏感信息检索、或自动代码依赖链搜索**。

### 1. ⚙️ GitHub Actions 自动化工作流配置

在您项目根目录创建 `.github/workflows/ace-search-audit.yml` 文件：

```yaml
name: Codebase Semantic Audit

on:
  push:
    branches: [ master ]
  pull_request:
    branches: [ master ]

jobs:
  semantic-audit:
    name: Run Semantic Audit Check
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Code
        uses: actions/checkout@v4

      - name: Setup Rust Toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Build ace-ctx
        run: cargo build --release

      # 语义检索自动审计
      - name: Audit for Insecure Coding Practices
        env:
          # 使用 GitHub Secrets 安全存储敏感凭证
          ACE_BASE_URL: ${{ secrets.ACE_BASE_URL }}
          ACE_TOKEN: ${{ secrets.ACE_TOKEN }}
        run: |
          echo "==== 开始对代码库进行语义安全审查 ===="
          
          # 通过 CLI 对代码库发起检索，检查是否存在硬编码凭证或不安全连接
          # 默认已排除 .md、README 等文档，只搜索源码
          ./target/release/ace-ctx \
            --search "are there any hardcoded credentials or insecure direct socket connections" > audit_report.txt

          cat audit_report.txt

      - name: Archive Audit Report
        uses: actions/upload-artifact@v4
        with:
          name: semantic-audit-report
          path: audit_report.txt
```

### 2. 🛡️ 为什么要在 CI 中使用？

1. **零外部依赖环境检索**：
   在 CI 环境中，我们通常不需要运行庞大的 MCP GUI 或者是交互式 Agent。利用 **`--search` 命令行特性**，CI 脚本可以用一条指令快速查询代码，并直接输出到日志或文件中，非常利于脚本自动化分析。
   
2. **过滤噪音，聚焦核心代码**：
   CLI 搜索默认排除文档类文件，只对核心源码进行扫描，从而使 CI 的语义审计报告**极度精准**。如需同时搜索文档，添加 `--include-document-files` 即可。

### 🔒 3. 机密安全管理说明（针对 CI/CD）

由于 `--search` 运行时必须配置 `--base-url` 和 `--token`（或对应环境变量 `ACE_BASE_URL` / `ACE_TOKEN`），这属于敏感鉴权凭证。**切记不要将其硬编码在 CI 配置文件中**：
* **在 GitHub 仓库中**：请前往仓库的 `Settings` -> `Secrets and variables` -> `Actions`，添加两个 Repository Secrets：
  - `ACE_BASE_URL`：您的检索服务后端 URL。
  - `ACE_TOKEN`：您的验证 token。
* **在工作流中**：使用 `${{ secrets.ACE_BASE_URL }}` 形式安全注入到环境变量中，如上文模板所示。这能保障您的代码在公开或私有 CI 中运行时，凭证绝不泄露。


## 故障排除

### 编译失败

1. 确保 Rust 版本足够新：`rustc --version`
2. 清理并重新编译：`cargo clean && cargo build --release`

### 运行时找不到命令

1. 确认二进制文件在 PATH 中：`which ace-ctx`
2. 确认文件有执行权限：`ls -la $(which ace-ctx)`
