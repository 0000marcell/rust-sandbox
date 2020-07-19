use tokio::time::{self, Duration};
use std::future::Future;

fn set_interval<F, Fut>(mut f: F, dur: Duration)
where
    F: Send + 'static + FnMut() -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    // Create stream of intervals.
    let mut interval = time::interval(dur);
    
    tokio::spawn(async move {
        // Skip the first tick at 0ms.
        interval.tick().await;
        loop {
            // Wait until next tick:
            interval.tick().await;
            // Spawn a task for the operation.
            tokio::spawn(f());
        }
    });
}

#[tokio::main]
async fn main() {
    set_interval(|| async {
        println!("Hello world!");
    }, Duration::from_millis(1000));
    
    time::delay_for(Duration::from_millis(6000)).await;
}
