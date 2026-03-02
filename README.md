<p align="center">
  <img src="unitest.png" width="140" height="140" alt="UnitestAgent Logo" />
</p>

<h1 align="center">UnitestAgent</h1>

<p align="center">
  <b>AI-Powered Unit Test Generation & Static Code Analysis Desktop App</b><br/>
  <b>AI 驱动的单元测试生成 & 静态代码分析桌面工具</b>
</p>

<p align="center">
  <a href="#-english">English</a> | <a href="#-中文">中文</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue?style=flat-square" alt="version" />
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=flat-square" alt="platform" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="license" />
  <img src="https://img.shields.io/badge/Tauri-1.2-orange?style=flat-square" alt="tauri" />
  <img src="https://img.shields.io/badge/Vue-3.2-42b883?style=flat-square&logo=vue.js&logoColor=white" alt="vue" />
  <img src="https://img.shields.io/badge/Rust-powered-dea584?style=flat-square&logo=rust&logoColor=white" alt="rust" />
  <img src="https://img.shields.io/badge/AI-LLM%20Powered-blueviolet?style=flat-square" alt="ai" />
</p>

---

<!-- ============================== ENGLISH ============================== -->

# English

## Why UnitestAgent?

Writing unit tests is essential for code quality — but it's time-consuming. **UnitestAgent** brings the power of large language models to your desktop, enabling you to:

- **Closed-Loop Engine** — Generate → Execute → Verify Coverage → Repair → Iterate automatically
- **Self-Healing Tests** — AI auto-repairs tests that fail to compile or run, no manual intervention needed
- **Coverage-Driven** — Automatically identify uncovered lines, iterate until target coverage is met
- **Mutation Testing Analysis** — Go beyond line coverage — detect surviving mutants and weak assertions
- **Quality Scoring** — AI evaluates test quality across 5 dimensions (coverage, assertions, mutation resistance, design, completeness)
- **Privacy-First** — Your code never leaves your machine, complete local data processing

> From manual test writing to a fully autonomous testing agent — **boost your testing productivity by 10x**.

---

## Key Features

### Closed-Loop Test Generation Engine

| Feature | Description |
|---------|-------------|
| **Coverage-Driven Loop** | Generate → Execute → Parse Coverage → Feedback → Re-generate, fully automated |
| **Self-Healing Tests** | When tests fail to compile or run, AI analyzes errors and auto-repairs them |
| **Smart Generation** | AI generates high-coverage unit tests based on source files, existing tests, and coverage gaps |
| **Coverage-Aware** | Supports Cobertura, Lcov, and Jacoco coverage reports for precise targeting |
| **Early Exit** | Automatically stops when desired coverage target is met, saving API tokens |
| **Iterative Feedback** | Failed test info and coverage data are fed back into prompts for smarter generation |
| **Context-Aware** | Import related files so the AI deeply understands code dependencies |

### Mutation Testing & Quality Scoring

| Feature | Description |
|---------|-------------|
| **Mutation Analysis** | Identify surviving mutants (AOR, ROR, COR, SDL, CR, RVM operators) |
| **Quality Scoring** | 5-dimension assessment: Coverage, Assertions, Mutation Resistance, Design, Completeness |
| **Weakness Detection** | Pinpoint exactly which code mutations would go undetected by current tests |
| **Actionable Suggestions** | Priority-ranked improvement suggestions with target line numbers |

### Static Code Analysis

| Feature | Description |
|---------|-------------|
| **Code Standards** | Detect naming violations, style issues, and redundant code |
| **Performance Diagnostics** | Identify potential bottlenecks and resource waste |
| **Security Scanning** | Detect common vulnerabilities — injection risks, out-of-bounds access, memory leaks |
| **Structured Reports** | Each issue includes ID, category, severity, location, impact analysis, and fix suggestions |

### Comprehensive Reports

- **Modern dashboard** — Beautiful report with coverage metrics, iteration tracking, and quality scores
- **Tabbed navigation** — Overview, Unit Tests, Static Analysis, and Coverage panels
- **Syntax highlighting** — Code rendering powered by Prism.js
- **Exportable** — HTML format reports, ready to download and share

### Multi-Model Support

| Model | Description |
|-------|-------------|
| DeepSeek-V3 | High-performance general reasoning |
| DeepSeek Reasoner | Deep logical reasoning |
| GPT-4o | OpenAI's latest multimodal model |
| Claude 3.7 Sonnet | Anthropic's advanced reasoning model |
| SiliconFlow | High-speed inference acceleration |
| Tencent Cloud DeepSeek | Stable access in China |
| Custom API | Any OpenAI-compatible endpoint |

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   Frontend                       │
│   Vue 3 + Naive UI + Pinia + Tailwind CSS       │
│   ┌──────────┐ ┌──────────┐ ┌───────────────┐  │
│   │  Chat UI │ │  Config  │ │ Report Viewer │  │
│   └────┬─────┘ └────┬─────┘ └──────┬────────┘  │
│        │            │               │            │
├────────┼────────────┼───────────────┼────────────┤
│        │      Tauri IPC Bridge      │            │
├────────┼────────────┼───────────────┼────────────┤
│                 Backend (Rust)                    │
│   ┌─────────────────────────────────────────┐   │
│   │          UnitestAgent Core              │   │
│   │  ┌───────────┐  ┌──────────────────┐   │   │
│   │  │ AI Caller │  │  Prompt Builder  │   │   │
│   │  │(Streaming)│  │  (Tera + TOML)   │   │   │
│   │  └───────────┘  └──────────────────┘   │   │
│   │  ┌───────────┐  ┌──────────────────┐   │   │
│   │  │ Coverage  │  │    Report        │   │   │
│   │  │ Processor │  │   Generator      │   │   │
│   │  └───────────┘  └──────────────────┘   │   │
│   └─────────────────────────────────────────┘   │
│                                                  │
│   ┌───────────┐ ┌─────────┐ ┌───────────────┐  │
│   │  Tokio    │ │ Reqwest │ │ langchain-rs  │  │
│   │  Async    │ │  HTTP   │ │   LLM SDK     │  │
│   └───────────┘ └─────────┘ └───────────────┘  │
└─────────────────────────────────────────────────┘
```

### Tech Stack

| Layer | Technology |
|-------|-----------|
| **Desktop Framework** | Tauri 1.2 — lightweight, secure, cross-platform |
| **Frontend** | Vue 3 + Vite 4 + TypeScript |
| **UI Components** | Naive UI + Tailwind CSS |
| **State Management** | Pinia |
| **i18n** | Vue I18n (en-US / zh-CN / zh-TW) |
| **Backend** | Rust + Tokio async runtime |
| **AI Integration** | langchain-rust + reqwest streaming |
| **Template Engine** | Tera (Jinja2-style) |
| **Report Rendering** | HTML + Prism.js syntax highlighting |
| **Coverage Parsing** | Cobertura / Lcov / Jacoco |

---

## Quick Start

### Prerequisites

- **Node.js** >= 18
- **pnpm** >= 8
- **Rust** >= 1.70
- **Tauri CLI** >= 1.2

### Installation

```bash
# 1. Clone the repository
git clone https://github.com/nicEDT/unitestagentapp.git
cd unitestagentapp

# 2. Install dependencies
pnpm install

# 3. Run in development mode
pnpm dev

# 4. Build for production
pnpm build
```

### Configure AI Model

Set up your API Key and model endpoint in the app settings:

| Setting | Description |
|---------|-------------|
| API Key | Your LLM service API key |
| API URL | Model service endpoint URL |
| Model | Select the model (DeepSeek / GPT-4o / Claude, etc.) |

---

## How It Works — Closed-Loop Engine  

```
                         ┌─────────────────────────────────────────────────────┐
                         │           UnitestAgent Closed-Loop Engine           │
                         └─────────────────────────────────────────────────────┘

  Source Code ──┐
                │    ┌───────────┐    ┌───────────┐    ┌──────────────┐
  Existing      │    │  Prompt   │    │ AI Model  │    │   Parse &    │
  Tests     ────┼──> │  Builder  │──> │(Streaming)│──> │   Validate   │
                │    │ +Coverage │    │           │    │   (YAML)     │
  Coverage  ────┘    └───────────┘    └───────────┘    └──────┬───────┘
       ▲                                                       │
       │                                                       ▼
       │                                              ┌──────────────┐
       │   ┌───────────┐    ┌───────────────┐         │  Write Tests │
       │   │  Coverage  │    │ Test Runner   │         │  to File     │
       └───│  Parser    │<───│ (Shell Exec)  │<────────└──────────────┘
           └─────┬─────┘    └───────┬───────┘
                 │                  │
                 │           ┌──────▼──────┐
                 │           │  Tests OK?  │──── Yes ──> Coverage Met? ─── Yes ──┐
                 │           └──────┬──────┘                                      │
                 │                  │ No                                           │
                 │           ┌──────▼──────┐                                      │
                 │           │ Self-Heal   │                                      │
                 │           │ AI Repair   │                                      │
                 │           └─────────────┘                                      │
                 │                                                                │
                 │    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   │
                 └──> │   Static     │──> │   Quality    │──> │   Report     │<──┘
                      │   Analysis   │    │   Scoring    │    │  Dashboard   │
                      │              │    │  + Mutation   │    │              │
                      └──────────────┘    └──────────────┘    └──────────────┘
```

---

## Supported Languages

UnitestAgent's language mapping covers **400+ programming languages and file types**, with core support for:

<table>
<tr>
<td>

**Systems**
- C / C++
- Rust
- Go
- Assembly

</td>
<td>

**Application**
- Python
- Java / Kotlin
- C# / .NET
- Swift

</td>
<td>

**Web**
- JavaScript / TypeScript
- Vue / React (JSX/TSX)
- PHP
- Ruby

</td>
<td>

**Others**
- Scala
- Elixir / Erlang
- Haskell
- Lua

</td>
</tr>
</table>

**Coverage Report Formats:** Cobertura XML | Lcov | Jacoco

---

## Highlights

| Feature | Description |
|---------|-------------|
| **Closed-Loop Engine** | Fully automated: Generate → Execute → Verify → Repair → Iterate |
| **Self-Healing Tests** | AI auto-repairs failing tests based on compiler/runtime error output |
| **Coverage-Driven** | Stops iterating only when target coverage is met, or max iterations reached |
| **Mutation Testing** | AI-powered mutation analysis to assess real fault-detection capability |
| **Quality Scoring** | 5-dimension test quality assessment (0-100) with improvement suggestions |
| **Desktop-First** | Built on Tauri — tiny installer, low resource usage, local data processing |
| **Streaming Response** | Real-time streaming output with conversational interaction |
| **Multi-Model Switching** | One-click switch between DeepSeek / GPT-4o / Claude and more |
| **Structured Output** | YAML-formatted test cases, easy to integrate into CI/CD pipelines |
| **Trilingual UI** | Simplified Chinese, Traditional Chinese, and English |
| **Privacy-First** | Local desktop app — your code never passes through third-party servers |
| **Cross-Platform** | macOS, Windows, and Linux |

---

## Project Structure

```
unitestagentapp/
├── src/                          # Vue frontend source
│   ├── api/                      # Tauri IPC call wrappers
│   ├── components/               # Shared components (settings, project mgmt)
│   ├── locales/                  # i18n resources (en-US / zh-CN / zh-TW)
│   ├── store/                    # Pinia state management
│   ├── views/unitest/            # Main UI (chat-style interface)
│   └── utils/                    # Utility functions
├── src-tauri/                    # Rust backend source
│   ├── src/app/uagent/           # AI Agent core logic
│   │   ├── ai_caller.rs          # LLM streaming calls
│   │   ├── unitest_agent.rs      # Agent orchestration engine
│   │   ├── prompt_builder.rs     # Prompt builder
│   │   ├── coverage_processor.rs # Coverage report parser
│   │   ├── report_generator.rs   # HTML report generator
│   │   └── code_analysis.rs      # Static code analysis
│   └── prompts/                  # TOML prompt templates
├── index.html                    # Entry page
├── vite.config.ts                # Vite build config
├── tailwind.config.js            # Tailwind CSS config
└── package.json                  # Project dependencies
```

---

## Roadmap

- [x] AI unit test generation
- [x] Static code analysis (standards / performance / security)
- [x] Multi-LLM provider support
- [x] Coverage report parsing (Cobertura / Lcov / Jacoco)
- [x] Unified HTML report generation
- [x] Internationalization (CN / EN / TW)
- [x] **Closed-loop engine: Generate → Execute → Coverage → Repair → Iterate**
- [x] **Self-healing test repair (AI auto-fix failing tests)**
- [x] **Test quality scoring system (5-dimension assessment)**
- [x] **Mutation testing analysis (surviving mutant detection)**
- [x] **Real-time progress tracking with phase-based status**
- [ ] CI/CD integration plugin
- [ ] Batch file test generation
- [ ] VS Code / JetBrains plugin
- [ ] History & version comparison
- [ ] Differential coverage gates for PR reviews
- [ ] Multi-agent orchestration (Generator + Validator + Reviewer)

---

## Contributing

Contributions, bug reports, and feature requests are welcome!

```bash
# After forking the repo
git checkout -b feature/your-feature
# After development
pnpm lint:fix
git commit -m "feat: your feature description"
git push origin feature/your-feature
# Submit a Pull Request
```

---

## License

[MIT License](license) - Copyright (c) 2026 Eric Du

---

---

<!-- ============================== 中文 ============================== -->

# 中文

## 为什么选择 UnitestAgent？

编写单元测试是保障代码质量的关键，但往往耗费大量时间。**UnitestAgent** 将 AI 大模型的能力带到你的桌面端，让你拥有一个**全自动的测试 Agent**：

- **闭环引擎** — 生成 → 执行 → 覆盖率验证 → 自修复 → 迭代，全流程自动化
- **测试自修复** — AI 自动分析编译/运行错误并修复失败测试，无需人工干预
- **覆盖率驱动** — 自动识别未覆盖代码行，迭代直至达到目标覆盖率后提前退出
- **变异测试分析** — 超越行覆盖率，检测存活变异体和薄弱断言
- **质量评分** — AI 从 5 个维度评估测试质量（覆盖率、断言、变异抵抗、设计、完整性）
- **隐私优先** — 本地桌面应用，代码数据不出机器

> 从手动编写测试到全自主测试 Agent，**将测试编写效率提升 10 倍**。

---

## 核心功能

### 闭环测试生成引擎

| 功能 | 说明 |
|------|------|
| **覆盖率驱动闭环** | 生成 → 执行 → 覆盖率解析 → 反馈 → 再生成，全流程自动化 |
| **测试自修复** | 测试编译/运行失败时，AI 自动分析错误并修复测试代码 |
| **智能测试生成** | 基于源文件、已有测试和覆盖率缺口，AI 自动生成高覆盖率单元测试 |
| **覆盖率感知** | 支持 Cobertura、Lcov、Jacoco 格式，精准定位未覆盖代码 |
| **提前退出** | 达到目标覆盖率自动停止迭代，节省 API Token |
| **迭代反馈** | 失败测试信息和覆盖率数据自动反馈到下一轮提示词 |
| **上下文关联** | 可引入关联文件，让 AI 更深入理解代码依赖关系 |

### 变异测试 & 质量评分

| 功能 | 说明 |
|------|------|
| **变异分析** | 识别存活变异体（AOR、ROR、COR、SDL、CR、RVM 算子） |
| **质量评分** | 5 维度评估：覆盖率、断言质量、变异抵抗力、设计质量、完整性 |
| **弱点检测** | 精准定位哪些代码变异不会被当前测试检测到 |
| **优化建议** | 按优先级排序的改进建议，附带目标行号 |

### 静态代码分析

| 功能 | 说明 |
|------|------|
| **代码规范检查** | 识别命名规范、代码风格、冗余代码等问题 |
| **性能问题诊断** | 发现潜在的性能瓶颈和资源浪费 |
| **安全漏洞扫描** | 检测常见安全隐患，如注入风险、越界访问、内存泄漏等 |
| **结构化报告** | 每个问题包含 ID、分类、严重等级、位置、影响分析和修复建议 |

### 综合报告

- **现代化仪表盘** — 覆盖率指标、迭代追踪、质量评分一目了然
- **标签页导航** — 总览、单元测试、静态分析、覆盖率四大面板
- **语法高亮** — 基于 Prism.js 的代码高亮渲染
- **可导出** — HTML 格式报告，支持下载和分享

### 多模型支持

| 模型 | 说明 |
|------|------|
| DeepSeek-V3 | 高性能通用推理 |
| DeepSeek Reasoner | 深度逻辑推理 |
| GPT-4o | OpenAI 最新多模态模型 |
| Claude 3.7 Sonnet | Anthropic 高级推理模型 |
| SiliconFlow | 高速推理加速 |
| 腾讯云 DeepSeek | 国内稳定访问 |
| 自定义 API | 支持任意 OpenAI 兼容接口 |

---

## 技术架构

```
┌─────────────────────────────────────────────────┐
│                   前端 (Frontend)                 │
│   Vue 3 + Naive UI + Pinia + Tailwind CSS       │
│   ┌──────────┐ ┌──────────┐ ┌───────────────┐  │
│   │ 对话界面  │ │ 配置面板  │ │  报告查看器   │  │
│   └────┬─────┘ └────┬─────┘ └──────┬────────┘  │
│        │            │               │            │
├────────┼────────────┼───────────────┼────────────┤
│        │      Tauri IPC 通信桥      │            │
├────────┼────────────┼───────────────┼────────────┤
│                 后端 (Rust)                       │
│   ┌─────────────────────────────────────────┐   │
│   │         UnitestAgent 核心引擎            │   │
│   │  ┌───────────┐  ┌──────────────────┐   │   │
│   │  │ AI 调用器 │  │  提示词构建器    │   │   │
│   │  │ (流式传输) │  │  (Tera + TOML)  │   │   │
│   │  └───────────┘  └──────────────────┘   │   │
│   │  ┌───────────┐  ┌──────────────────┐   │   │
│   │  │ 覆盖率    │  │    报告          │   │   │
│   │  │ 解析器    │  │   生成器         │   │   │
│   │  └───────────┘  └──────────────────┘   │   │
│   └─────────────────────────────────────────┘   │
│                                                  │
│   ┌───────────┐ ┌─────────┐ ┌───────────────┐  │
│   │  Tokio    │ │ Reqwest │ │ langchain-rs  │  │
│   │  异步运行  │ │  HTTP   │ │   LLM SDK     │  │
│   └───────────┘ └─────────┘ └───────────────┘  │
└─────────────────────────────────────────────────┘
```

### 技术栈一览

| 层级 | 技术 |
|------|------|
| **桌面框架** | Tauri 1.2 — 轻量、安全、跨平台 |
| **前端** | Vue 3 + Vite 4 + TypeScript |
| **UI 组件** | Naive UI + Tailwind CSS |
| **状态管理** | Pinia |
| **国际化** | Vue I18n（中/英/繁） |
| **后端** | Rust + Tokio 异步运行时 |
| **AI 调用** | langchain-rust + reqwest 流式传输 |
| **模板引擎** | Tera（Jinja2 风格） |
| **报告渲染** | HTML + Prism.js 语法高亮 |
| **覆盖率解析** | Cobertura / Lcov / Jacoco |

---

## 快速开始

### 环境要求

- **Node.js** >= 18
- **pnpm** >= 8
- **Rust** >= 1.70
- **Tauri CLI** >= 1.2

### 安装步骤

```bash
# 1. 克隆仓库
git clone https://github.com/nicEDT/unitestagentapp.git
cd unitestagentapp

# 2. 安装依赖
pnpm install

# 3. 开发模式运行
pnpm dev

# 4. 构建生产版本
pnpm build
```

### 配置 AI 模型

在应用设置中配置你的 API Key 和模型端点：

| 配置项 | 说明 |
|--------|------|
| API Key | 你的 LLM 服务 API 密钥 |
| API URL | 模型服务端点地址 |
| Model | 选择使用的模型（DeepSeek / GPT-4o / Claude 等） |

---

## 工作流程 — 闭环反馈引擎

```
                         ┌─────────────────────────────────────────────────────┐
                         │          UnitestAgent 闭环反馈引擎                   │
                         └─────────────────────────────────────────────────────┘

  源代码 ────┐
             │    ┌───────────┐    ┌───────────┐    ┌──────────────┐
  已有测试 ──┼──> │ 提示词     │──> │ AI 模型   │──> │  解析 & 校验  │
             │    │ 构建器     │    │ (流式传输) │    │  (YAML)      │
  覆盖率 ───┘    │ +覆盖率    │    │           │    └──────┬───────┘
       ▲         └───────────┘    └───────────┘           │
       │                                                   ▼
       │                                          ┌──────────────┐
       │   ┌───────────┐    ┌───────────────┐     │  写入测试文件  │
       │   │  覆盖率    │    │  测试执行器    │     │              │
       └───│  解析器    │<───│  (Shell 命令)  │<───└──────────────┘
           └─────┬─────┘    └───────┬───────┘
                 │                  │
                 │           ┌──────▼──────┐
                 │           │  测试通过？   │── 是 ──> 覆盖率达标？ ── 是 ──┐
                 │           └──────┬──────┘                               │
                 │                  │ 否                                    │
                 │           ┌──────▼──────┐                               │
                 │           │  AI 自修复  │                               │
                 │           │  错误分析    │                               │
                 │           └─────────────┘                               │
                 │                                                         │
                 │    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
                 └──> │  静态代码分析 │──> │  质量评分     │──> │  报告仪表盘   │<─┘
                      │              │    │  + 变异测试   │    │              │
                      └──────────────┘    └──────────────┘    └──────────────┘
```

---

## 使用指南

### 1. 创建项目配置

在应用中创建一个新项目，填写以下信息：

- **源文件路径** — 待测试的源代码文件
- **测试文件路径** — 现有的测试文件（可选）
- **覆盖率报告** — 已有的覆盖率报告路径（可选）
- **测试运行命令** — 执行测试的 Shell 命令
- **最大迭代次数** — AI 生成测试的迭代轮数

### 2. 生成单元测试

点击 **测试执行**，AI Agent 将：

1. 读取源文件和已有测试代码
2. 分析覆盖率报告，定位未覆盖代码行
3. 构建上下文丰富的提示词
4. 流式调用大模型生成测试用例
5. 输出结构化的 YAML 测试结果
6. 多轮迭代直至覆盖率达标

### 3. 静态代码分析

点击 **静态扫描**，系统将自动：

1. 对源代码进行全面的静态分析
2. 从代码规范、性能、安全三个维度检查
3. 生成包含详细修复建议的结构化报告

### 4. 查看报告

生成完成后，系统自动产出一份综合 HTML 报告，包含：

- 测试用例列表及其详细信息
- 静态分析发现的问题及修复建议
- 语法高亮的代码片段

---

## 支持的语言

UnitestAgent 的语言映射覆盖 **400+ 种编程语言和文件类型**，核心支持包括：

<table>
<tr>
<td>

**系统级语言**
- C / C++
- Rust
- Go
- Assembly

</td>
<td>

**应用级语言**
- Python
- Java / Kotlin
- C# / .NET
- Swift

</td>
<td>

**Web 开发**
- JavaScript / TypeScript
- Vue / React (JSX/TSX)
- PHP
- Ruby

</td>
<td>

**其他**
- Scala
- Elixir / Erlang
- Haskell
- Lua

</td>
</tr>
</table>

**覆盖率报告格式支持：** Cobertura XML | Lcov | Jacoco

---

## 特色亮点

| 特性 | 说明 |
|------|------|
| **闭环引擎** | 全自动：生成 → 执行 → 验证 → 修复 → 迭代 |
| **测试自修复** | AI 根据编译/运行错误自动修复失败测试 |
| **覆盖率驱动** | 达到目标覆盖率自动停止，节省 Token 和时间 |
| **变异测试** | AI 变异分析评估真实的错误检测能力 |
| **质量评分** | 5 维度测试质量评估（0-100），附带改进建议 |
| **桌面优先** | 基于 Tauri 构建，安装包体积小、资源占用低，数据本地处理 |
| **流式响应** | 实时流式输出生成进度，对话式交互体验 |
| **多模型切换** | 一键切换 DeepSeek / GPT-4o / Claude 等模型 |
| **结构化输出** | YAML 格式测试用例，便于集成到 CI/CD 流水线 |
| **三语支持** | 界面支持简体中文、繁体中文、English |
| **隐私安全** | 本地桌面应用，代码数据不经过第三方服务器 |
| **跨平台** | 支持 macOS、Windows、Linux |

---

## 项目结构

```
unitestagentapp/
├── src/                          # Vue 前端源码
│   ├── api/                      # Tauri IPC 调用封装
│   ├── components/               # 通用组件（设置、项目管理等）
│   ├── locales/                  # 国际化资源（en-US / zh-CN / zh-TW）
│   ├── store/                    # Pinia 状态管理
│   ├── views/unitest/            # 主界面（对话式 UI）
│   └── utils/                    # 工具函数
├── src-tauri/                    # Rust 后端源码
│   ├── src/app/uagent/           # AI Agent 核心逻辑
│   │   ├── ai_caller.rs          # LLM 流式调用
│   │   ├── unitest_agent.rs      # Agent 编排引擎
│   │   ├── prompt_builder.rs     # 提示词构建器
│   │   ├── coverage_processor.rs # 覆盖率解析器
│   │   ├── report_generator.rs   # HTML 报告生成
│   │   └── code_analysis.rs      # 静态代码分析
│   └── prompts/                  # TOML 提示词模板
├── index.html                    # 入口页面
├── vite.config.ts                # Vite 构建配置
├── tailwind.config.js            # Tailwind CSS 配置
└── package.json                  # 项目依赖
```

---

## 路线图

- [x] AI 单元测试生成
- [x] 静态代码分析（规范 / 性能 / 安全）
- [x] 多 LLM 提供商支持
- [x] 覆盖率报告解析（Cobertura / Lcov / Jacoco）
- [x] 综合 HTML 报告生成
- [x] 国际化（中 / 英 / 繁）
- [x] **闭环引擎：生成 → 执行 → 覆盖率 → 修复 → 迭代**
- [x] **测试自修复（AI 自动修复失败测试）**
- [x] **测试质量评分系统（5 维度评估）**
- [x] **变异测试分析（存活变异体检测）**
- [x] **实时进度追踪（阶段化状态推送）**
- [ ] CI/CD 集成插件
- [ ] 批量文件测试生成
- [ ] VS Code / JetBrains 插件
- [ ] 历史记录与版本对比
- [ ] PR 差异覆盖率门禁
- [ ] 多 Agent 协同（生成器 + 验证器 + 审查器）

---

## 贡献

欢迎贡献代码、报告问题或提出功能建议！

```bash
# Fork 仓库后
git checkout -b feature/your-feature
# 开发完成后
pnpm lint:fix
git commit -m "feat: your feature description"
git push origin feature/your-feature
# 提交 Pull Request
```

---

## 许可证

[MIT License](license) - Copyright (c) 2024 Eric Du

---

<p align="center">
  <b>UnitestAgent</b> — Let AI safeguard your code | 让 AI 为你的代码保驾护航
</p>
