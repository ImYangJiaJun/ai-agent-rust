use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_stream::stream;
use backon::{ExponentialBuilder, Retryable};
use futures::{Stream, StreamExt};

fn chat_complete_stream(//不应该是异步的
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {//使用yield输出流
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

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(message)
            .max_tokens(2048u32)
            .build()?;

        let mut stream=client.chat().create_stream(request).await?;//发出流式请求

        while let Some(response_result) = stream.next().await{//使用循环处理每一块新到达的数据
            match response_result{
                Ok(chunk)=>{
                    if let Some(choice) = chunk.choices.first()
                    &&  let Some(new_text) = &choice.delta.content{//delta是增量内容
                        yield Ok(new_text.clone());
                    }
                }
                Err(err)=> yield Err(err.into())
            }
        }
    }
}

pub async fn chat_stream_with_retry(model: &str, system: Option<&str>, prompt: &str)->anyhow::Result<String>{
    let op = || async {//重试需要能够反复调用，Future await一次就没有了，但是闭包每一次都会创建一个新的Future
        let s = chat_complete_stream(model, system, prompt);

        futures::pin_mut!(s);
        let mut output = String::new();
        while let Some(result) = s.next().await {
            match result {
                Ok(txt) => {
                    output.push_str(&txt);
                    println!("{txt}");
                }
                Err(err) => {
                    tracing::error!("\nError while stream: {err}");
                    return Err(err);
                }
            }
        }
        Ok(output)
    };

    op.retry(ExponentialBuilder::default().with_max_times(3)).await//指数级重试策略（第几次重试就等待2的n减一次方秒）
}
