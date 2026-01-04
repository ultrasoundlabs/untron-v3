use anyhow::Result;
use std::{
    future::Future,
    time::{Duration, Instant},
};
use tokio::time;
use tokio_util::sync::CancellationToken;

pub async fn await_or_cancel<T>(
    shutdown: &CancellationToken,
    fut: impl Future<Output = Result<T>>,
) -> Result<Option<T>> {
    tokio::select! {
        _ = shutdown.cancelled() => Ok(None),
        res = fut => Ok(Some(res?)),
    }
}

pub async fn timed_await_or_cancel<T, Fut>(
    shutdown: &CancellationToken,
    fut: Fut,
) -> Result<Option<(T, u64)>>
where
    Fut: Future<Output = Result<T>>,
{
    let start = Instant::now();
    let res = crate::shared::r#async::await_or_cancel(shutdown, fut).await?;
    let Some(value) = res else {
        return Ok(None);
    };
    Ok(Some((value, start.elapsed().as_millis() as u64)))
}

pub async fn sleep_or_cancel(shutdown: &CancellationToken, duration: Duration) -> Result<()> {
    tokio::select! {
        _ = shutdown.cancelled() => Ok(()),
        _ = time::sleep(duration) => Ok(()),
    }
}
