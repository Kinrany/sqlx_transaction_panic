use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let conn = MySqlPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let handle = tokio::spawn(tokio::time::sleep(Duration::from_secs(2)));

    let mut tx = conn.begin().await.unwrap();
    sqlx::query("SELECT 1").fetch_one(&mut tx).await.unwrap();
    drop(tx);

    handle.await.unwrap();
}
