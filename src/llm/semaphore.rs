//限流,这里是简化写法，最好是根据供应商的策略写

use std::sync::OnceLock;
use tokio::sync::Semaphore;//异步的信号量，用于控制一次最多执行几个任务

static SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

pub fn get_semaphore() -> &'static Semaphore {
    SEMAPHORE.get_or_init(|| Semaphore::new(3))//全局并发限流，最多3个
}