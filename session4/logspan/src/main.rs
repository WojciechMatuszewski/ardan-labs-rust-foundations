use tracing_subscriber;
use tracing;
use tracing_subscriber::fmt::format::FmtSpan;

#[tracing::instrument]
async fn hello_world() {
    println!("Hello!")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE | FmtSpan::EXIT)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up");
    tracing::warn!("Are you sure?");
    tracing::error!("Something went wrong");

    hello_world().await;

    return Ok(());
}
