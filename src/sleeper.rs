use std::time::Duration;

#[async_trait::async_trait]
pub trait Sleeper {
    async fn sleep(duration: Duration);
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl Sleeper for crate::DefaultSleeper {
    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}
