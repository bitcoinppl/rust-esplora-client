#[cfg(feature = "tokio")]
use std::time::Duration;

pub trait Runtime {
    fn sleep(duration: Duration) -> impl std::future::Future<Output = ()> + Send;
}

pub struct DefaultRuntime;

#[cfg(feature = "tokio")]
impl Runtime for DefaultRuntime {
    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

#[cfg(feature = "async-std")]
pub struct AsyncStd;

#[cfg(feature = "async-std")]
impl Runtime for AsyncStd {
    async fn sleep(duration: Duration) {
        async_std::task::sleep(duration).await;
    }
}
