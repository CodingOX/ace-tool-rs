# ace-ctx

[English](README.md) | 简体中文

一个高性能的 CLI 代码库上下文引擎，专为 AI 智能体（AI Agents）与开发者设计，使用 Rust 编写，支持通过 **Skill 技能包** 驱动极速、低损耗的终端代码库语义检索。

> [!IMPORTANT]
> **特别致谢**：本项目 fork 自原项目 [missdeer/ace-tool-rs](https://github.com/missdeer/ace-tool-rs)。在此向原作者 `missdeer` 的无私奉献与杰出付出致以最诚挚的谢意！本项目在此基础上进行了 AI 智能体场景（特别是 Skill 驱动）的深度适配与功能扩展。

---

## 🌟 核心亮点：AI 智能体 Skill 集成

如果您正在使用支持引入 `Skill` 技能包的下一代智能体编程框架（如 **Claude Code**、**Antigravity IDE** 等），`ace-ctx` 为您提供了一套高度优化、随时随地可直接导入的 AI Agent 专属技能包！

### 1. 技能包存放路径
该 Skill 文件存放于项目中的：
👉 [skills/ace-code-search-expert/SKILL.md](skills/ace-code-search-expert/SKILL.md)

### 2. 工作原理与优势（前置上下文收窄）
智能体一旦导入此 Skill，便会自动捕获您的代码检索意图，并在后台静默调用全局的 `ace-ctx --search` 命令行工具。
- **极速检索**：直接在本地和远程索引服务进行向量匹配，无需大模型盲目遍历。
- **节省 Token**：智能体只把提炼出的 100% 靠谱的代码切片作为精简上下文，极大节省 Token 消耗和上下文空间。
- **杜绝幻觉**：精准的代码行号和文件定位，彻底杜绝模型因缺乏局部上下文而产生幻觉。

### 3. 如何为您的 Agent 导入它？
1. **设置环境变量**：配置系统全局变量 `ACE_BASE_URL` 和 `ACE_TOKEN`。
2. **导入技能**：将 [skills/ace-code-search-expert/SKILL.md](skills/ace-code-search-expert/SKILL.md) 文件的完整路径复制，并导入到您的 AI 智能体工作空间的 Skill 定义目录中。
   > [!NOTE]
   > 智能体在执行该技能时会自动进行环境嗅探：若检测到本地未安装 `ace-ctx` 命令行工具，它将通过 `npx -y` 自动免安装运行，无需您手动全局安装。当然，为了获得极致的运行速度，您也可以选择通过 `npm install -g @alistar.max/ace-ctx` 进行全局安装。

---

## 💻 命令行直接检索 (CLI Search Mode)

配置好环境变量后，您可以在终端中直接调用 `ace-ctx` 运行极速的语义检索：

```bash
# 对当前项目进行自然语言检索（默认源码优先，自动排除文档带来的噪音）
ace-ctx --search "用户登录时是如何连接数据库的？"

# 若检索确实需要包含文档（如 .md, .txt 等）
ace-ctx --search "如何部署和安装本项目" --include-document-files
```

检索结果会直接以高度匹配、带有高亮和精准行号的代码切片形式输出到标准输出（stdout），极其利于终端阅读与 AI 智能体解析。

---

## 🔧 极简配置与安装

### 1. 快速开始（推荐）
使用 npx 是安装和运行 `ace-ctx` 最简单的方式：
```bash
npx @alistar.max/ace-ctx --base-url <API_URL> --token <AUTH_TOKEN>
```
这会自动下载适合您平台的二进制文件（支持 Windows、macOS、Linux）。

### 2. 环境变量快捷配置
为了避免每次命令行运行都要重复输入繁琐的参数，建议在您的系统环境变量中配置它们：
```bash
# macOS/Linux (Zsh/Bash)
export ACE_BASE_URL="https://api.example.com"
export ACE_TOKEN="your-token-here"

# 配置后即可极简运行：
ace-ctx --search "查找用户模块"
```

### 3. 从源码构建
```bash
git clone https://github.com/CodingOX/ace-ctx.git
cd ace-ctx
cargo build --release
# 二进制文件位于 target/release/ace-ctx
```

---

## ⚡ 其他辅助运行模式

除了核心的 **CLI 语义检索模式**，`ace-ctx` 还支持以下辅助能力：

1. **项目代码单次增量索引模式 (Index-Only Mode)**
   扫描当前项目提取增量变化文件并上传向量索引，完成后立即安全退出，非常适合集成在 Git Commit Hook 或 CI 自动化部署流程中。
   ```bash
   ace-ctx --index-only
   ```
2. **终端 Prompt 增强模式 (Enhance-Prompt Mode)**
   在终端快速改写、优化并补充您输入的 Prompt，使其带有完美的本地代码上下文。
   ```bash
   ace-ctx --enhance-prompt "重构用户认证模块"
   ```
3. **MCP 服务模式 (Model Context Protocol)**
   作为标准的 MCP 服务器在后台运行，为兼容 MCP 的客户端提供 `search_context` 和 `enhance_prompt` 接口。
   ```bash
   ace-ctx --base-url <API_URL> --token <AUTH_TOKEN>
   ```

---

## 📄 许可证说明

本项目采用双许可证模式：
- **非商业 / 个人使用**：采用 [GNU General Public License v3.0](LICENSE)。
- **商业 / 工作场所使用**：在商业环境、工作场所中使用或用于任何商业目的，需获取商业许可证。商业许可证咨询请联系：`missdeer@gmail.com`。

---

## 👥 贡献与支持

欢迎提交 Pull Request 和 Issue！
- 原项目贡献者：[missdeer](https://github.com/missdeer)

[![Star History Chart](https://starchart.cc/CodingOX/ace-ctx.svg)](https://starchart.cc/CodingOX/ace-ctx)
