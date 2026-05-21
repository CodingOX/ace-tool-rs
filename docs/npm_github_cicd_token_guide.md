# 🚀 NPM 与 GitHub 自动化 CI/CD Token 配置指南

本指南详细介绍了如何为项目配置 **NPM Automation Token** 以及 **GitHub Personal Access Token (PAT)**，以实现在 GitHub Actions 中的全自动、无干预包发布流程。

---

## 🎯 核心原理与背景

在现代 CI/CD 流程中，GitHub Actions 容器是运行在云端的无交互式环境。如果您的 NPM 账户启用了双重身份验证 (2FA/MFA)，普通的发布 Token 在发布时会因无法手动输入验证码而导致部署流崩溃。

为此，我们需要：
1. **在 NPM 创建 "Automation" 类型的 Token**：专门为 CI/CD 自动部署设计，能够在保证账户安全的同时**绕过双重认证 (2FA)** 顺利发布。
2. **在 GitHub 创建自定义访问 Token**：用于给 CI/CD 工作流赋予修改仓库、提交 Tag 以及发布 Release 的高级权限。

---

## 🔑 第一部分：创建 NPM 自动化部署 Token (`NPM_TOKEN`)

随着 NPM 官方页面的升级，最新版的 Token 创建步骤如下：

### 📝 详细操作步骤：

1. **登录官网**：打开并登录 [npmjs.com](https://www.npmjs.com/)。
2. **进入 Token 管理页**：点击右上角个人头像，在下拉菜单中选择 **"Access Tokens"**。
3. **生成新 Token**：点击页面右上角的 **"Generate New Token"** 按钮。
   > [!IMPORTANT]
   > 最新版网页会提供两种 Token 生成方式：**Granular Access Token** (细粒度 Token) 和 **Classic Token** (传统 Token)。
   > **推荐选择 "Classic Token"** 以获得最稳健的 CI/CD 兼容性。

4. **配置 Token 属性**：
   * **Name**: 起一个有辨识度的名字，例如 `ace-ctx-github-cicd`。
   * **Type**: ⚠️ **必须选择 "Automation" 选项**！
     > [!WARNING]
     > * **Automation**：专为无交互式 CI/CD 设计。即使您的账户开启了 2FA 强制验证，此 Token 也可以在发布时免除 2FA 校验。
     > * **Publish**：适用于手动发布，会在发布时拦截并要求输入 2FA 六位验证码，不适合云端流水线。
     > * **Read-Only**：仅能下载和读取包，无法进行任何发布操作。

5. **复制并保存**：点击 **"Generate Token"** 后，页面会展示生成的 Token。
   > [!CAUTION]
   > **请立即复制并妥善保存此 Token！** 一旦刷新或离开该页面，NPM 将再也不会展示这个 Token 的明文。

---

## 🛡️ 第二部分：创建 GitHub 访问 Token (`GH_TOKEN`)

虽然 GitHub Actions 默认会提供一个临时的 `GITHUB_TOKEN`，但它在很多场景下权限受限（例如无法触发其他由 workflow 依赖的钩子，或者在推回 Tag 时受阻）。为了稳健运行，建议创建一个自定义的 **Personal Access Token (PAT)**。

### 📝 详细操作步骤：

1. **进入开发者设置**：登录 GitHub，点击右上角头像 -> **Settings** -> 滚动到最左侧下方选择 **Developer settings**。
2. **选择 Token 类型**：选择 **Personal access tokens** -> **Tokens (classic)**。
3. **创建新 Token**：点击右上角 **Generate new token** -> 选择 **Generate new token (classic)**。
4. **配置权限范围 (Scopes)**：
   * **Note**: 起个名字，例如 `ace-ctx-cicd-pusher`。
   * **Expiration**: 推荐选择 `No expiration` (无过期时间，方便长期自动化运行) 或设定一个较长的时间。
   * **Select Scopes (勾选权限)**：
     * `[x] repo` (完整控制仓库：包含代码提交、Tag 上传、Release 发布)
     * `[x] workflow` (允许更新 GitHub Actions 工作流文件)
5. **保存 Token**：点击最下方的 **"Generate token"**，复制并妥善保存生成的 `ghp_...` 字符。

---

## ⚙️ 第三部分：在 GitHub 中配置项目 Secrets

拥有这两个 Tokens 后，我们需要把它们安全地托管在 GitHub 仓库中，让 GitHub Actions 可以作为环境变量免密读取。

### 📝 详细操作步骤：

1. 打开您的 GitHub 仓库页面（例如 `CodingOX/ace-ctx`）。
2. 点击顶部导航栏的 **Settings** (设置) 按钮。
3. 在左侧边栏中，展开 **Secrets and variables** -> 选择 **Actions**。
4. 点击右上角的 **New repository secret** 按钮来依次添加这两个 Token：

#### 1. 配置 NPM 部署密钥
* **Name**: `NPM_TOKEN` (必须与您的 CI 工作流中的引用名一致)
* **Secret**: 粘贴您在第一部分生成的 **NPM Automation Token**。

#### 2. 配置 GitHub 访问密钥
* **Name**: `GH_TOKEN`
* **Secret**: 粘贴您在第二部分生成的 **GitHub PAT Classic Token** (`ghp_...`)。

---

## 🤖 第四部分：在 GitHub Actions 工作流中进行消费

在您的项目中，创建 `.github/workflows/publish.yml` 文件。以下是消费这两个 Secrets 自动发布 Scoped 包的具体流程示例：

```yaml
name: Publish to NPM

on:
  push:
    tags:
      - 'v*' # 只有当我们推送 v1.0.0 这种版本 tag 时才触发

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Code
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GH_TOKEN }} # 使用自定义 GH_TOKEN 获得写权限

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          registry-url: 'https://registry.npmjs.org' # 设置注册表为 NPM

      # 关键步骤：在 Actions 容器中通过环境变量自动写入 NPM 认证凭据
      - name: Configure NPM Authentication
        run: |
          echo "//registry.npmjs.org/:_authToken=${{ secrets.NPM_TOKEN }}" > ~/.npmrc
        env:
          NPM_TOKEN: ${{ secrets.NPM_TOKEN }}

      # 发布我们的主 Scoped 包
      - name: Publish Scoped Package
        run: |
          cd npm/ace-ctx
          npm publish --access public
```

---

## 💡 终极自检与排错 checklist

* **NPM 403 Forbidden 报错**：如果您发布的是 Scoped 包（例如 `@alistar.max/ace-ctx`），首次发布时 NPM 默认会将其判定为私有包。由于免费账号无法发布私有作用域包，必须显式在发布命令后附加参数 `--access public`（如上面的示例所示），否则会被拒绝发布。
* **NPM 2FA 拦截错误**：请二次确认在 NPM 创建 Token 时选择的是 **"Automation"** 类型的 Classic Token。如果是 `Publish` 类型的 Token，发布流水线依然会被挂起并报错。
* **GitHub Actions 推送 tag 权限被拒**：请确保在 Actions 流程中使用 `secrets.GH_TOKEN` 进行 checkout，否则默认的 GITHUB_TOKEN 可能会因为权限受限而拒绝推送版本发布产物。
