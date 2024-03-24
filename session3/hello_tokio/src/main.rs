async fn hello() -> u32 {
    return 3;
}

async fn hello2() -> u32 {
    return 4;
}

#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(ticker()),
        tokio::spawn(hello2()),
        tokio::spawn(ticker()),
    );
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}
