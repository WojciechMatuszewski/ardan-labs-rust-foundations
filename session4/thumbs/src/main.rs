use anyhow::anyhow;
use axum::extract::{Multipart, Path};
use axum::http::{header, HeaderMap};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Router};
use futures::TryStreamExt;
use sqlx::Row;
use tokio::task::spawn_blocking;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().expect("Failed to load the environment variables");
    let db_url = dotenv::var("DATABASE_URL").expect("Missing DATABASE_URL variable");

    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    fill_missing_thumbnails(&pool).await?;

    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .route("/image/:id", get(get_image))
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

async fn get_image(Path(id): Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}.jpg");
    let attachment = format!("filename={filename}");

    let file = tokio::fs::File::open(&filename).await.unwrap();

    return axum::response::Response::builder()
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/jpeg"),
        )
        .header(
            header::CONTENT_DISPOSITION,
            header::HeaderValue::from_str(&attachment).unwrap(),
        )
        .body(axum::body::Body::from_stream(ReaderStream::new(file)))
        .unwrap();
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
        spawn_blocking(move || return make_thumbnail(new_image_id))
            .await
            .unwrap()
            .unwrap();
    } else {
        panic!("Missing field")
    }

    return "Ok".to_string();
}

async fn fill_missing_thumbnails(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    let mut rows = sqlx::query("select id from images").fetch(pool);

    while let Some(row) = rows.try_next().await? {
        let id = row.get::<i64, _>(0);

        spawn_blocking(move || return make_thumbnail(id)).await??;
    }

    return Ok(());
}

fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let thumbnail_path = format!("images/{id}_thumb.jpg");
    if std::path::Path::new(&thumbnail_path).exists() {
        return Ok(());
    }

    let image_path = format!("images/{id}.jpg");
    let image_bytes = std::fs::read(image_path)?;

    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };

    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;

    return Ok(());
}
