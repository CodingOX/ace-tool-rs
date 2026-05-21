---
name: ace-code-search-expert
description: >
  Perform semantic code search, locate files, retrieve code snippets, analyze
  codebase architecture, or narrow down code context. You MUST use this skill
  whenever the user asks to find, locate, search, or understand code behavior
  — even when they don't explicitly say "search" (e.g., "where is X
  implemented", "how does Y work", "find the code that handles Z"). Works via
  ace-ctx CLI. If CLI is unavailable, fall back to ripgrep/grep without
  blocking. Ensure ace-ctx or @alistar.max/ace-ctx is available.
---

# ACE Code Search Expert

此技能作为自主上下文收窄前置处理流，拦截用户与代码库相关的提示，通过 `ace-ctx` 执行语义搜索，将结果提炼为高相关性代码上下文，再进入后续编码或推理阶段。

---

## 前置条件检查

调用 `ace-ctx` 前必须完成以下检查。任一条件不满足则**跳过本技能**，使用 ripgrep/grep 替代：

1. **环境变量**：确认 `ACE_BASE_URL` 和 `ACE_TOKEN` 均已设置且非空
   - 可检查项目 `.env` 文件自动加载
   - 缺少任一变量时立即退出，提示用户设置，**禁止盲目执行**
2. **CLI 可用性**：
   - **优先方式**：`ace-ctx` — 若已全局安装，直接运行最快（安装：`npm install -g @alistar.max/ace-ctx`）
   - **兜底方式**：若未检测到全局 `ace-ctx`，自动用 `npx -y @alistar.max/ace-ctx` 免安装运行

> **命名规范**：始终使用完整名称 `ace-ctx`（或 `@alistar.max/ace-ctx`），不可简写为 `ace`。

---

## 核心工作流

### 1. 搜索决策
- **Direct Search（首选）**：用户提示包含明确的开发意图或代码特征时，直接使用 `--search` 进行语义检索
- **Conditional Enhancement**：仅在输入极度简短/抽象时先用 `--enhance-prompt` 强化 query，再执行检索

### 2. Query 构造规范
```
[自然语言描述具体设计或流程逻辑] Keywords: [关键字1, 关键字2, 关键字3]
```

### 3. 文档排除规则
- **默认 `exclude_document_files=true`**：过滤 `.md`/`.txt`/`README`/`CHANGELOG` 等语义噪声，聚焦源码
- **例外**：仅在用户明确要求检索文档内容时设为 `false`

### 4. 结果提炼与输出
检索返回后必须经过收窄处理：

**双重清洗**：
- 剔除不相关类、重复代码、琐碎日志，仅保留核心逻辑
- 追踪上下游依赖调用链（如 Model → Controller → Service），简要勾勒交互流

**绝对路径输出**：
- 所有文件引用必须使用可点击的 Markdown 绝对路径：
  `[basename](file:///absolute/path/to/file#Lstart-Lend)`
- 链接文字不加反引号
- 示例：[main.rs](file:///absolute/path/to/project/src/main.rs#L89-L188)

---

## 快速参考

### CLI 命令
```bash
# 语义搜索（核心用法）
npx @alistar.max/ace-ctx --search "query" --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN"

# 仅索引（首次使用需先索引项目）
npx @alistar.max/ace-ctx --index-only --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN"

# 已全局安装后可省略 npx
ace-ctx --search "query" --base-url "$ACE_BASE_URL" --token "$ACE_TOKEN"
```

### 环境变量设置
```bash
export ACE_BASE_URL="https://your-api-server.com"
export ACE_TOKEN="your-secure-token"
```
> Windows PowerShell: `$env:ACE_BASE_URL="..."` / CMD: `set ACE_BASE_URL=...`
