# ai-agent-rust

使用 Rust 开发 AI Agent 的入门项目，跟随 B 站 up 主「软件工艺师」的视频教程开发：

- 【使用 Rust 开发 AI Agent - 简介】<https://www.bilibili.com/video/BV1qHEQ6RERo/>

本项目使用[硅基流动（SiliconFlow）](https://cloud.siliconflow.cn/i/11QqSVoZ)提供的**免费模型**作为 LLM 后端，通过 OpenAI 兼容接口（`async-openai`）调用，演示了 LLM 的普通对话与结构化输出（JSON Schema）两种能力，是搭建 Agent 的基础模块。

## 功能特性

- **普通对话**：`chat_complete`，将 system + user 消息发送给模型并返回文本回复
- **结构化输出**：`chat_complete_structured`，借助 `schemars` 从 Rust 结构体自动生成 JSON Schema，以 `response_format: json_schema`（strict 模式）强制模型输出符合 `ActionPlan` 结构的 JSON，为 Agent 的"规划 → 行动"提供可靠的数据基础
- **异步友好**：基于 tokio，模块划分清晰，便于继续扩展工具调用（function calling）等 Agent 能力

## 技术栈

| 组件 | 用途 |
| ---- | ---- |
| Rust（2024 edition） | 编程语言 |
| tokio | 异步运行时 |
| async-openai | OpenAI 兼容 API 客户端（`chat-completion` feature） |
| schemars | 从 Rust 结构体自动生成 JSON Schema |
| serde / serde_json | 序列化 / 反序列化 |
| dotenvy | 加载 `.env` 环境变量 |
| tracing / tracing-subscriber | 日志输出 |

## 快速开始

### 1. 环境要求

- Rust 工具链（建议使用 rustup 安装最新 stable 版本）
- [硅基流动](https://cloud.siliconflow.cn/i/11QqSVoZ)账号及 API Key（免费）

### 2. 配置环境变量

复制 `.env.temp` 为 `.env` 并填写：

```bash
OPENAI_BASE_URL=https://api.siliconflow.cn/v1
OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxx
```

> `.env` 已被 `.gitignore` 忽略，请勿提交你的 API Key。

### 3. 运行

```bash
cargo run
```

默认示例会调用结构化输出，让模型为「去美加墨世界杯观看比赛」生成一份 `ActionPlan`（目标、步骤、难度、预估耗时）并打印。

## 模型说明（重要）

模型常量定义在 `src/constant.rs`：

| 常量 | 模型 ID | 结构化输出（作者实测） |
| ---- | ------- | ---------- |
| `GLM_4_9B` | `THUDM/GLM-4-9B-0414` | ✅ 可以 |
| `GLM_Z1_9B` | `THUDM/GLM-Z1-9B-0414` | ❌ 不可以 |

> ⚠️ **注意**：以上结论仅基于作者对几个硅基流动免费模型的实测，不代表模型本身的能力上限。
> 在作者尝试过的免费模型中，**`THUDM/GLM-4-9B-0414`** 可以正常使用 `src/llm/structured.rs` 中基于 `response_format: json_schema` 的结构化输出；
> 推理模型 `THUDM/GLM-Z1-9B-0414` 实测会忽略该参数、输出整篇 Markdown 正文，无法用于结构化解析。
> 如果你打算换用其他模型，请自行验证其是否支持 JSON Schema / 结构化输出，否则需要回退到提示词约束 + 容错解析的兜底方案。

## 项目结构

```
src/
├── main.rs                  # 入口：演示普通对话与结构化输出
├── lib.rs                   # 库模块声明
├── constant.rs              # 模型常量定义
├── models/
│   └── action_plan.rs       # ActionPlan / ActionStep / Difficulty 数据模型
└── llm/
    ├── complete.rs          # chat_complete：普通对话
    └── structured.rs        # chat_complete_structured：结构化输出
```

## License

MIT
