use async_std::stream::StreamExt;
use async_std::sync::channel;
use async_std::task;

use std::thread;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let (requests_tx, requests_rx) = channel::<i32>(10);
    let mut requests_rx = requests_rx.throttle(Duration::from_secs(1));

    thread::spawn(move || {
        task::block_on( async {
            for i in 0..5 {
                requests_tx.send(i).await;
            }
        });

        thread::sleep(Duration::from_secs(100));
    });

    while let Some(item) = requests_rx.next().await {
        println!("{:?}", item);
    }
}
