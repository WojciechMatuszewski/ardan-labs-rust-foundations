use std::time::Duration;
use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} started");

    // tokio::time::sleep(Duration::from_millis(time)).await;

    let _ = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    }).await;
    
    println!("Task {task} finished");
}

#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 500),
        hello_delay(2, 1000),
        hello_delay(3, 1500),
    );
}
