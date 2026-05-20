# 🛠️ ace-tool CLI & CI 使用指南

`ace-tool` 不仅可以作为 Claude Code / IDE 的 MCP (Model Context Protocol) 服务端，更可以通过您新添加的两个核心特性，作为一个独立的 **CLI 命令行工具** 使用，并能无缝集成到 **CI (持续集成)** 工作流中，进行自动化的语义检索与代码静态审查。

本指南将结合您新增加的以下两个特性，为您提供详尽的 CLI 使用说明与 CI 集成方案：
1. 🔍 **特性一：`--search` 终端命令行语义检索**
2. 🛡️ **特性二：文档与特定文件动态过滤排除 (`SearchFilterOptions`)**

---

## 💻 一、CLI (命令行界面) 使用说明

### 1. 本地构建与安装
首先，确保您已经在本地构建或安装了 `ace-tool` 二进制可执行文件。

在项目根目录下执行：
```bash
# 构建 Release 版本
cargo build --release

# 或者直接安装到您的本地 cargo 二进制目录（通常在 ~/.cargo/bin）
cargo install --path .
```
安装完成后，您可以在终端直接通过 `ace-tool` 运行命令。

### 2. CLI 基础语义检索
利用您新增的 `--search` 特性，您可以在终端无需启动 MCP 服务而直接对代码库进行自然语言搜索。

#### 📌 基本语法
```bash
ace-tool \
  --base-url "<您的后端 API 地址>" \
  --token "<您的 API 访问令牌>" \
  --search "<您的自然语言检索词>"
```

#### 💡 实用示例
* **查找数据库连接初始化代码**：
  ```bash
  ace-tool --base-url "https://api.ace-search.com" --token "my-secure-token" --search "where do we initialize the database connections"
  ```
* **搜索错误处理的最佳实践**：
  ```bash
  ace-tool --base-url "https://api.ace-search.com" --token "my-secure-token" --search "how are custom errors handled in this codebase"
  ```

---

### 3. CLI 中如何结合“动态文档/文件过滤”特性
目前，您的动态过滤特性 (`SearchFilterOptions`) 默认会在 CLI 搜索中初始化并编译 Glob 规则。

#### MCP 配置文件级过滤
如果您通过 MCP 模式（例如集成在 Claude Code 或 cursor 中）运行，您可以在配置文件（如 `.claude/settings.json` 或 MCP 配置文件）中，直接以工具参数形式传入过滤条件：
```json
{
  "mcpServers": {
    "ace-tool": {
      "command": "ace-tool",
      "args": ["--base-url", "https://api.ace.com", "--token", "secret"],
      "always_allow": ["search_context"],
      "settings": {
        "exclude_document_files": true,
        "exclude_extensions": [".txt", ".csv"],
        "exclude_globs": ["**/temp/**", "docs/**"]
      }
    }
  }
}
```

#### 💡 进阶：如何让 CLI 也直接支持命令行过滤参数？ (可选扩展)
如果您希望让命令行直接在终端就能带上排除过滤参数，您的底层 `SearchFilterOptions` 已经完全支持，您只需要在 `src/main.rs` 的 `Args` 结构体中加几行 `clap` 参数定义即可：
```rust
    /// Exclude documentation files from search (.md, .txt, etc.)
    #[arg(long)]
    exclude_doc: bool,

    /// Exclude specific extensions (comma-separated, e.g. "txt,csv")
    #[arg(long, value_delimiter = ',')]
    exclude_ext: Vec<String>,

    /// Exclude specific glob patterns (comma-separated, e.g. "docs/**,temp/**")
    #[arg(long, value_delimiter = ',')]
    exclude_globs: Vec<String>,
```
*(底层逻辑只需要将这几个参数传入到 `SearchFilterOptions` 中即可，这能为您提供无与伦比的命令行自由度！)*

---

## 🤖 二、CI (持续集成) 使用与配置说明

将 `ace-tool` 集成到 CI（例如 GitHub Actions、GitLab CI）中，可以在代码每次推送或提 PR 时，**自动对代码库进行合规性审查、敏感信息检索、或自动代码依赖链搜索**。

这里结合您的“文档过滤”和“命令行搜索”特性，为您提供一份 **GitHub Actions** 的最佳实践模板：

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
        with:
          cache: true

      - name: Build ace-tool
        run: cargo build --release

      # 结合您的最新特性进行语义检索与自动审计
      - name: Audit for Insecure Coding Practices
        env:
          # 使用 GitHub Secrets 安全存储敏感凭证
          ACE_BASE_URL: ${{ secrets.ACE_BASE_URL }}
          ACE_TOKEN: ${{ secrets.ACE_TOKEN }}
        run: |
          echo "==== 开始对代码库进行语义安全审查 ===="
          
          # 通过 CLI 对代码库发起检索，检查是否存在硬编码凭证或不安全连接
          # 由于我们主要检查源码，在 CI 中过滤掉 .md、README 等文档能大幅提高检索效率
          ./target/release/ace-tool \
            --base-url "$ACE_BASE_URL" \
            --token "$ACE_TOKEN" \
            --search "are there any hardcoded credentials or insecure direct socket connections" > audit_report.txt

          cat audit_report.txt

      - name: Archive Audit Report
        uses: actions/upload-artifact@v4
        with:
          name: semantic-audit-report
          path: audit_report.txt
```

---

### 2. 🛡️ 为什么要结合您的两个新特性在 CI 中使用？

1. **零外部依赖环境检索**：
   在 CI 环境中，我们通常不需要运行庞大的 MCP GUI 或者是交互式 Agent。利用您的 **`--search` 命令行特性**，CI 脚本可以用一条指令快速查询代码，并直接输出到日志或文件中，非常利于脚本自动化分析。
   
2. **过滤噪音，聚焦核心代码**：
   在 CI 自动化安全扫描或依赖链搜索时，大量的 markdown 文档、CHANGELOG、LICENSE 会对检索的相关度带来“噪音干扰”。利用您开发的 **文档和 Glob 排除过滤特性**，能让 CI 检索引擎自动避开文档及临时生成文件，只对核心源码进行扫描，从而使 CI 的语义审计报告**极度精准**。

---

### 🔒 3. 机密安全管理说明（针对 CI/CD）
由于 `--search` 运行时必须配置 `--base-url` 和 `--token`，这属于敏感鉴权凭证。**切记不要将其硬编码在 CI 配置文件中**：
* **在 GitHub 仓库中**：请前往仓库的 `Settings` -> `Secrets and variables` -> `Actions`，添加两个 Repository Secrets：
  - `ACE_BASE_URL`：您的检索服务后端 URL。
  - `ACE_TOKEN`：您的验证 token。
* **在工作流中**：使用 `${{ secrets.ACE_BASE_URL }}` 形式安全注入到环境变量中，如上文模板所示。这能保障您的代码在公开或私有 CI 中运行时，凭证绝不泄露。
