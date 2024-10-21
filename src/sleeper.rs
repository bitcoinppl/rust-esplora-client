use std::time::Duration;

pub trait Sleeper {
    fn sleep(duration: Duration) -> impl std::future::Future<Output = ()> + Send;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultSleeper;

#[cfg(feature = "tokio")]
impl Sleeper for DefaultSleeper {
    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}
