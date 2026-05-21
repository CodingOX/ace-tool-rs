---
name: ace-code-search-expert
description: Use when you need to perform semantic code searches, locate files, retrieve code snippets, analyze codebase architecture/features, or narrow down code context. Ensure that the '@alistar.max/ace-tool-rs' package or 'ace-tool-rs' CLI is available.
---

# ACE Code Search Expert Skill

This skill acts as an autonomous, pre-processing context-narrowing pipeline (上下文收窄前置处理流)。Its core purpose is to intercept user prompts concerning the codebase, execute highly targeted semantic searches using `search_context` via the `ace-tool-rs` CLI tool (distributed via NPM package `@alistar.max/ace-tool-rs`), and distill the results into a compact, high-relevance code context before any subsequent coding or reasoning takes place.

> [!IMPORTANT]
> **工具分发、自动就绪与规范命名**：
> 1. **规范命名**：本工具已正式命名并确定为 **`ace-tool-rs`**。在所有 CLI 调用和终端执行中，**必须**完整且精确地使用 `ace-tool-rs`（或通过 npx 调用时使用 `@alistar.max/ace-tool-rs`），绝不能简写为 `ace` 或 `ace-tool`。
> 2. **NPM 优先与免安装运行**：本工具已采用 NPM 统一分发。推荐智能体或用户优先通过 **`npx`** 免安装运行（例如 `npx @alistar.max/ace-tool-rs [options]`）。
> 3. **自动安装就绪机制**：若当前环境未检测到全局 `ace-tool-rs` 命令，智能体**应当自动在终端尝试执行全局安装**：
>    ```bash
>    npm install -g @alistar.max/ace-tool-rs
>    ```
>    或者直接在每次调用时通过 `npx -y @alistar.max/ace-tool-rs` 运行。若因环境缺失 Node.js/NPM 导致均无法就绪，此时应执行快速退出并引导用户。

---

## 💻 CLI 运行与模式说明

通过 NPM 部署后，您可以使用以下两种命令格式之一来执行 `ace-tool-rs`：
1. **免安装直接运行 (npx)**：直接在命令前加上 `npx` 即可运行，最为便捷省心（推荐）。
2. **全局安装后运行 (Global CLI)**：首次使用需在终端执行全局安装命令：
   ```bash
   npm install -g @alistar.max/ace-tool-rs
   ```
   安装成功后，后续便可直接在任何目录下使用命令 `ace-tool-rs`。

以下是三种独立命令行模式在两种执行格式下的具体调用范例：

- **项目代码单次索引模式 (Index-Only Mode)**：
  解析当前项目代码并上传向量索引，完成后立即退出。
  ```bash
  # 方式 A：npx 免安装执行（推荐）
  npx @alistar.max/ace-tool-rs --index-only --base-url <BASE_URL> --token <TOKEN>

  # 方式 B：全局安装后执行（需先执行 npm install -g @alistar.max/ace-tool-rs）
  ace-tool-rs --index-only --base-url <BASE_URL> --token <TOKEN>
  ```
- **终端直接语义检索模式 (Search Mode)**：
  对代码库进行自然语言检索并输出匹配的代码切片。
  ```bash
  # 方式 A：npx 免安装执行（推荐）
  npx @alistar.max/ace-tool-rs --search "检索词/问题描述" --base-url <BASE_URL> --token <TOKEN>

  # 方式 B：全局安装后执行（需先执行 npm install -g @alistar.max/ace-tool-rs）
  ace-tool-rs --search "检索词/问题描述" --base-url <BASE_URL> --token <TOKEN>
  ```
- **终端 Prompt 增强模式 (Enhance-Prompt Mode)**：
  在终端快速优化输入的 Prompt 并启动增强 Review。
  ```bash
  # 方式 A：npx 免安装执行（推荐）
  npx @alistar.max/ace-tool-rs --enhance-prompt "原始模糊指令" --base-url <BASE_URL> --token <TOKEN>

  # 方式 B：全局安装后执行（需先执行 npm install -g @alistar.max/ace-tool-rs）
  ace-tool-rs --enhance-prompt "原始模糊指令" --base-url <BASE_URL> --token <TOKEN>
  ```

---

## 🔑 环境变量配置与强力约束 (Environment Variables & Guard Rules)

在运行 `ace-tool-rs` CLI 之前，智能体**必须首先检查当前系统环境变量中是否已正确配置 `ACE_BASE_URL` 和 `ACE_TOKEN`**：

> [!CAUTION]
> **强力约束规则 (Guard Clause)**：
> 1. **前置条件验证**：在调用 `ace-tool-rs` 命令行前，智能体必须优先确认环境变量 `ACE_BASE_URL` 和 `ACE_TOKEN` 均已在当前环境中存在且不为空。
> 2. **自动设置与读取**：智能体应主动检查项目根目录下是否存在 `.env` 文件。若存在且含有相关配置，应首先尝试加载。
> 3. **快速失败与退出机制**：如果发现其中任何一个环境变量**未配置或为空**，且无法自动读取，智能体**必须立即放弃使用本技能**，直接退出并友好提示用户：“该工具当前无法调用，因为没有配置环境变量 `ACE_BASE_URL` 或 `ACE_TOKEN`。已选择使用其他替代方式（如 ripgrep / grep）进行探索。您可以执行 `export ACE_BASE_URL=...` 和 `export ACE_TOKEN=...` 快速设置。”
> 4. **禁止盲目尝试**：绝不允许在缺少环境变量时尝试在终端执行 `ace-tool-rs` 命令，这会引发报错并增加执行噪声。

### 1. 正确设置环境变量的方法

根据你的操作系统和 Shell，使用以下对应的方式设置环境变量：

#### 🍏 macOS / 🐧 Linux (Bash/Zsh)
```bash
export ACE_BASE_URL="https://your-api-server.com"
export ACE_TOKEN="your-secure-token"
```

#### 🪟 Windows (PowerShell)
```powershell
$env:ACE_BASE_URL="https://your-api-server.com"
$env:ACE_TOKEN="your-secure-token"
```

#### 🪟 Windows (CMD)
```cmd
set ACE_BASE_URL=https://your-api-server.com
set ACE_TOKEN=your-secure-token
```

### 2. 命令行调用方式

设置好环境变量后，你可以使用以下对应平台的形式执行 CLI 命令。得益于 NPM 包分发机制，在 Windows、macOS 和 Linux 上，**均统一使用 `ace-tool-rs` 或者是 `npx @alistar.max/ace-tool-rs` 作为标准命令**，无需再追加任何平台特定的后缀（如 `.exe`）。

#### 🍏 macOS / 🐧 Linux
```bash
# 语义检索
npx @alistar.max/ace-tool-rs --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN" --search "检索词"

# 项目索引
npx @alistar.max/ace-tool-rs --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN" --index-only
```

#### 🪟 Windows (PowerShell)
```powershell
# 语义检索
npx @alistar.max/ace-tool-rs --base-url $env:ACE_BASE_URL --token $env:ACE_TOKEN --search "检索词"

# 项目索引
npx @alistar.max/ace-tool-rs --base-url $env:ACE_BASE_URL --token $env:ACE_TOKEN --index-only
```

#### 🪟 Windows (CMD)
```cmd
# 语义检索
npx @alistar.max/ace-tool-rs --base-url %ACE_BASE_URL% --token %ACE_TOKEN% --search "检索词"

# 项目索引
npx @alistar.max/ace-tool-rs --base-url %ACE_BASE_URL% --token %ACE_TOKEN% --index-only
```

---

## 🎯 核心使用规则

### 1. Autonomous Pre-Processing Workflow (无交互前置流)
- 自动运行整个 "检索-收窄" 流程，不要因为检索中间结果向用户发送确认或中断提问。在完成代码切片提炼后，一气呵成直接进入下一个编码或分析阶段。

### 2. Search Decision Tree (条件增强决策)
- **Direct Search (首选直接检索)**: 若 Prompt 中包含明确的开发意图或代码特征，**跳过** `enhance_prompt`，直接使用 `search_context`（在 CLI 中为 `--search`）进行语义检索。
- **Conditional Enhancement (模糊提示词增强)**: **仅在**输入极其简短、抽象或宽泛时，才使用 `--enhance-prompt` 对其进行上下文强化，然后基于强化后的 query 执行语义检索。

### 3. Query Formulation Pattern (检索词构造规范)
必须使用以下特定结构组织 `query` 参数：
`[用自然语言描述你想在代码中找到的具体设计或流程逻辑] Keywords: [关键字1, 关键字2, 关键字3]`

### 4. Document Exclusion Rule (默认源码检索过滤)
- **直击源码 (Default True)**: 当智能体通过 MCP `search_context` 工具检索代码时，**必须默认将 `exclude_document_files` 设为 `true`**。这将自动剔除项目中的 `.md`、`.txt`、`README`、`CHANGELOG` 等文档产生的语义噪声，使搜索结果 100% 聚焦在源码实现本身。
- **例外场景 (Explicit Documentation Search)**: 只有在用户明确发出涉及文档内容的指令（例如“检索项目文档”、“查看 README 的使用说明”、“阅读 CHANGELOG”等）时，方可将 `exclude_document_files` 设置为 `false`。

---

## 📊 提炼并生成高可靠度 Context 规范 (Context Quality Guide)

检索结果返回后，必须通过以下收窄标准，确保最终沉淀 of Context 是 100% 完善和靠谱的，避免产生“检索幻觉”或上下文噪音：

### 1. 双重清洗与依赖流梳理 (Clean & Trace)
- **剔除噪音**：剔除检索返回的不相关类、重复代码及琐碎的辅助性日志输出，仅保留核心逻辑。
- **依赖关联**：寻找核心类周边的上下游依赖调用链（如 Model -> Controller -> Service），并在输出 the 交互流中简明勾勒出来。

### 2. 输出 100% 完善可直接点击的绝对路径 (Interactive Links)
- 整理目标文件时，**严禁只提供相对路径或模糊的文件名**。
- 必须输出成能够让当前用户在本地 IDE 中直接点击跳转的标准 Markdown 绝对路径格式：`[basename](file:///absolute/path/to/file#Lstart-Lend)`（注：不要在链接文字加反引号，避免破坏富文本解析）。
- *示例*（使用脱敏的通用绝对路径结构举例）：[main.rs](file:///absolute/path/to/project/src/main.rs#L89-L188)
