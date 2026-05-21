---
name: ace-code-search-expert
description: Use when you need to perform semantic code searches, locate files, retrieve code snippets, analyze codebase architecture/features, or narrow down code context. Ensure that the 'ace-tool-rs' CLI tool is globally available in the current environment; if not available, skip this skill.
---

# ACE Code Search Expert Skill

This skill acts as an autonomous, pre-processing context-narrowing pipeline (上下文收窄前置处理流)。Its core purpose is to intercept user prompts concerning the codebase, execute highly targeted semantic searches using `search_context` via the globally available `ace-tool-rs` CLI tool, and distill the results into a compact, high-relevance code context before any subsequent coding or reasoning takes place.

> [!IMPORTANT]
> **前提条件与工具命名**：
> 1. 本工具已正式命名并确定为 **`ace-tool-rs`**。在所有 CLI 调用和终端执行中，**必须**完整且精确地使用 `ace-tool-rs`，绝不能简写为 `ace` 或 `ace-tool`。
> 2. 本技能依赖于全局可用的 `ace-tool-rs` 命令行工具。若当前环境中未安装 `ace-tool-rs`，请直接跳过（Skip）本技能。


---

## 💻 CLI 运行与模式说明

假设 `ace-tool-rs` 工具已在全局环境就绪（Windows 平台下通常是 `ace-tool-rs.exe`），你可以在终端直接使用以下三种独立命令行模式：

- **项目代码单次索引模式 (Index-Only Mode)**：
  解析当前项目代码并上传向量索引，完成后立即退出。
  ```bash
  ace-tool-rs --index-only --base-url <BASE_URL> --token <TOKEN>
  ```
- **终端直接语义检索模式 (Search Mode)**：
  对代码库进行自然语言检索并输出匹配的代码切片。
  ```bash
  ace-tool-rs --search "检索词/问题描述" --base-url <BASE_URL> --token <TOKEN>
  ```
- **终端 Prompt 增强模式 (Enhance-Prompt Mode)**：
  在终端快速优化输入的 Prompt 并启动增强 Review。
  ```bash
  ace-tool-rs --enhance-prompt "原始模糊指令" --base-url <BASE_URL> --token <TOKEN>
  ```

---

## 🔑 环境变量密钥配置与强力约束 (Environment Variables & Guard Rules)

在运行 `ace-tool-rs` CLI 之前，智能体**必须首先检查当前系统环境变量中是否已正确配置 `ACE_BASE_URL` 和 `ACE_TOKEN`**：

> [!CAUTION]
> **强力约束规则 (Guard Clause)**：
> 1. **前置条件验证**：在调用 `ace-tool-rs` 命令行前，智能体必须优先确认环境变量 `ACE_BASE_URL` 和 `ACE_TOKEN` 均已在当前环境中存在且不为空。
> 2. **快速失败与退出机制**：如果发现其中任何一个环境变量**未配置或为空**，本工具**将完全无法调用**。智能体必须**立即放弃使用本技能，直接退出并通知用户/大模型：“该工具当前无法调用，因为没有配置环境变量 `ACE_BASE_URL` 或 `ACE_TOKEN`。已选择使用其他替代方式（如 ripgrep / grep）进行探索。”**
> 3. **禁止盲目尝试**：绝不允许在缺少环境变量时尝试在终端执行 `ace-tool-rs` 命令，这会引发报错并增加执行噪声。

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

设置好环境变量后，你可以使用以下对应平台的形式执行 CLI 命令。在 Windows 环境下，工具的可执行文件名通常是 `ace-tool-rs.exe`，如果已加入系统 `PATH`，可直接用 `ace-tool-rs` 或 `ace-tool-rs.exe` 调用。

#### 🍏 macOS / 🐧 Linux
```bash
# 语义检索
ace-tool-rs --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN" --search "检索词"

# 项目索引
ace-tool-rs --index-only --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN"
```

#### 🪟 Windows (PowerShell)
```powershell
# 语义检索 (可使用 ace-tool-rs 或 ace-tool-rs.exe)
ace-tool-rs.exe --base-url $env:ACE_BASE_URL --token $env:ACE_TOKEN --search "检索词"

# 项目索引
ace-tool-rs.exe --index-only --base-url $env:ACE_BASE_URL --token $env:ACE_TOKEN
```

#### 🪟 Windows (CMD)
```cmd
# 语义检索 (可使用 ace-tool-rs 或 ace-tool-rs.exe)
ace-tool-rs.exe --base-url %ACE_BASE_URL% --token %ACE_TOKEN% --search "检索词"

# 项目索引
ace-tool-rs.exe --index-only --base-url %ACE_BASE_URL% --token %ACE_TOKEN%
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

---

## 🤖 CI (持续集成) 自动化审计

`ace-tool-rs` 的 `--search` 功能非常适合无缝集成到 CI 工作流（如 GitHub Actions）中，用于在推送或 PR 时对代码库进行自动合规性审查与敏感信息检索：

```yaml
# GitHub Actions 示例片段
- name: Audit for Insecure Coding Practices
  env:
    ACE_BASE_URL: ${{ secrets.ACE_BASE_URL }}
    ACE_TOKEN: ${{ secrets.ACE_TOKEN }}
  run: |
    ace-tool-rs \
      --base-url "$ACE_BASE_URL" \
      --token "$ACE_TOKEN" \
      --search "are there any hardcoded credentials or insecure direct socket connections" > audit_report.txt
```
*注意：在 CI/CD 中，切记通过 Secrets 形式安全管理 `ACE_BASE_URL` 和 `ACE_TOKEN` 环境变量，绝不能硬编码在配置文件中。*

---

## 📊 提炼并生成高可靠度 Context 规范 (Context Quality Guide)

检索结果返回后，必须通过以下收窄标准，确保最终沉淀 of Context 是 100% 完善和靠谱的，避免产生“检索幻觉”或上下文噪音：

### 1. 双重清洗与依赖流梳理 (Clean & Trace)
- **剔除噪音**：剔除检索返回的不相关类、重复代码及琐碎的辅助性日志输出，仅保留核心逻辑。
- **依赖关联**：寻找核心类周边的上下游依赖调用链（如 Model -> Controller -> Service），并在输出 the 交互流中简明勾勒出来。

### 2. 输出 100% 完善可直接点击的绝对路径 (Interactive Links)
- 整理目标文件时，**严禁只提供相对路径或模糊的文件名**。
- 必须输出成 IDE 可直接跳转的标准 Markdown 绝对路径格式：`[basename](file:///absolute/path/to/file#Lstart-Lend)`（注：不要在链接文字加反引号，避免破坏富文本解析）。
- *示例*：[main.rs](file:///Users/alistar/code-all/ai/ace-tool-rs/src/main.rs#L89-L188)

### 3. 画出拓扑流动图 (Topology Flow)
- 在输出 Context 摘要时，**强烈建议使用 Mermaid 状态机或流程图**展示代码切片之间的通信流程，使用户/客户端一眼看穿整个业务实现，确保架构逻辑的可视化与完善性。
- *示例*：
  ```mermaid
  graph TD
      A[Args Parse] -->|Search Mode| B[IndexManager]
      B -->|Query| C[search_context]
  ```
