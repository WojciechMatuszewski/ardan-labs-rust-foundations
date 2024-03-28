use futures::TryStreamExt;
use sqlx::{Row, FromRow};

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn update_message(id: i64, message: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("update messages set message = ? where id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;

    return Ok(());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().expect("Failed to read the env file");

    let db_url = std::env::var("DATABASE_URL").expect("Failed to read env");

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // let messages = sqlx::query("select id, message from messages")
    //     .map(|row: sqlx::sqlite::SqliteRow| {
    //         let id: i64 = row.get(0);
    //         let message: String = row.get(1);
    //         return (id, message);
    //     }).fetch_all(&pool).await?;
    //
    // for (id, message) in messages {
    //     println!("{id}: {message}");
    // }

    update_message(4, "Updated Message", &pool).await?;

    let messages = sqlx::query_as::<_, Message>("select id, message from messages").fetch_all(&pool).await?;

    println!("{messages:?}");

    let mut message_stream = sqlx::query_as::<_, Message>("select id, message from messages").fetch(&pool);

    while let Some(message) = message_stream.try_next().await? {
        println!("{message:?}");
    }

    return Ok(());
}
