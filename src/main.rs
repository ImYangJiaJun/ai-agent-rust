use ai_agent_rust::constant::GLM_4_9B;
use tokio::task::JoinSet;
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;
use ai_agent_rust::llm::semaphore::get_semaphore;
use ai_agent_rust::llm::stream::chat_stream_with_retry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // 对话模式，发送一次等着全部返回
    // let content=chat_complete(GLM_Z1_9B,Some("你是一个全能助手"),"中国历史最强的乒乓球选手是谁").await?;
    // println!("{}", content);

    // 结构化输出
    // let plan = chat_complete_structured(
    //     GLM_4_9B,
    //     Some("你是一个全能助手"),
    //     "我要去美加墨世界杯观看比赛，如何安排？",
    // )
    // .await?;
    // println!("{:#?}", plan);

    let prompts = vec![
        "用三句话解释 Rust 的所有权机制",
        "什么是异步编程，和多线程有什么区别",
        "解释一下 TCP 三次握手的过程",
        "用简单的话概括什么是大语言模型",
        "Rust 中 Arc 和 Rc 的区别是什么",
        "什么是 RAG，为什么 AI 应用里常用它",
        "解释 HTTP 和 HTTPS 的区别",
        "什么是死锁，怎么避免",
        "用生活比喻解释什么是递归",
        "为什么说 Rust 没有 GC 但内存还是安全的",
    ];

    let mut set = JoinSet::new();
    for prompt in prompts {
        let span = tracing::trace_span!("Chat", prompt = prompt);
        set.spawn(
            async move {
                tracing::trace!("\n\n{prompt}");
                let permit = get_semaphore().acquire().await.unwrap();//请求到并发许可，许可了就继续执行，没有许可就挂起等待
                let output = chat_stream_with_retry(
                    GLM_4_9B,
                    Some("你是一个全能助理"),
                    prompt
                ).await.unwrap();
                drop(permit);//手动还回许可
                Ok::<_,anyhow::Error>((prompt, output))
            }.instrument(span)
        );
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt,result))) => tracing::info!("\n{prompt}\n{result}"),
            Ok(Err(err)) => tracing::error!("Task panicked: {err}"),
            Err(err) => tracing::error!("Task panicked: {err}"),
        }
    }

    Ok(())
}
