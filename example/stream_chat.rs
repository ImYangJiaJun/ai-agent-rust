// use ai_agent_rust::constant::GLM_4_9B;
// use ai_agent_rust::llm::stream::chat_complete_stream;
// use futures::StreamExt;
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     dotenvy::dotenv()?;
//
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::INFO)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber)?;
//
//     let s = chat_complete_stream(GLM_4_9B, Some("你是一个全能助手"), "从1数到10000");
//     futures::pin_mut!(s);
//     let mut output = String::new();
//     while let Some(result) = s.next().await {
//         match result {
//             Ok(txt) => {
//                 output.push_str(&txt);
//                 print!("{}", txt);
//             }
//             Err(err) => {
//                 tracing::error!("\nError while streaming: {}", err);
//                 return Err(err);
//             }
//         }
//     }
//
//     println!("Result {output}");
//     println!("------------------------------");
//
//     Ok(())
// }
