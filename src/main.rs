use ai_agent_rust::constant::GLM_4_9B;
use ai_agent_rust::llm::structured::chat_complete_structured;
use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // let content=chat_complete(GLM_Z1_9B,Some("你是一个全能助手"),"中国历史最强的乒乓球选手是谁").await?;
    // println!("{}", content);

    let plan=chat_complete_structured(
        GLM_4_9B,
        Some("你是一个全能助手"),
        "我要去美加墨世界杯观看比赛，如何安排？"
    ).await?;
    println!("{:#?}", plan);

    Ok(())
}
