use anyhow::anyhow;
use axum::extract::Multipart;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Router};
use sqlx::Row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().expect("Failed to load the environment variables");
    let db_url = dotenv::var("DATABASE_URL").expect("Missing DATABASE_URL variable");

    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .layer(Extension(pool));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    return Ok(());
}

async fn index_page() -> Html<&'static str> {
    const HTML: &str = include_str!("index.html");
    return Html(HTML);
}

async fn insert_image_into_database(pool: &sqlx::SqlitePool, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("insert into images (tags) values (?) returning id")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    return Ok(row.get(0));
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    let base_path = std::path::Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        anyhow::bail!("File already exists");
    }

    tokio::fs::write(image_path, bytes).await?;

    return Ok(());
}

async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart,
) -> String {
    let mut tags = None;
    let mut image = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()),
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) {
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
        save_image(new_image_id, &image).await.unwrap();
    } else {
        panic!("Missing field")
    }

    return "Ok".to_string();
}
