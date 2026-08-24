use async_openai::types::chat::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema};
use crate::models::action_plan::{ActionPlan, ActionStep};

pub async fn chat_complete_structured(model:&str, system:Option<&str>, prompt:&str) ->anyhow::Result<ActionPlan>{
    let client=async_openai::Client::new();
    let mut message=vec![];

    if let Some(system) = system {
        message.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()?
                .into()
        );
    }

    message.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into()
    );

    let schema = schemars::schema_for!(ActionPlan);     // 扫描 ActionStep 生成 Schema 类型
    let schema_json = schema.as_value().clone();          // 转换为 json 类型
    let format_setting = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema{
            description: Some("A step-by-step agent action plan with difficulty and time estimate".into()),// 结构化结果的描述
            name: "action_plan".into(),
            schema: schema_json,
            strict: Some(true),// 严格模式，所有字段必填，同时不允许大模型多输出
        }
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(message)
        .response_format(format_setting)
        .max_tokens(2048u32)
        .build()?;

    let response=client.chat().create(request).await?;

    tracing::info!("response={:#?}", response);

    let plan=response
        .choices
        .into_iter()
        .next()
        .and_then(|c|c.message.content)
        .ok_or_else(|| anyhow::anyhow!("No content in response"))
        .and_then(|s|serde_json::from_str(&s).map_err(Into::into))?;

    Ok(plan)
}