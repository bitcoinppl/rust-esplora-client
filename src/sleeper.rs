use std::time::Duration;

#[async_trait::async_trait]
pub trait Sleeper {
    async fn sleep(duration: Duration);
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultSleeper;

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl Sleeper for DefaultSleeper {
    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}
