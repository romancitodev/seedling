use seedling::SqlxExecutor;
use seedling::fake;
use seedling::generate;

#[cfg(feature = "sqlx")]
#[tokio::main]
async fn main() {
    use sqlx::Executor;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(sqlx::query(
        "CREATE TABLE users (id TEXT PRIMARY KEY NOT NULL, username TEXT NOT NULL, email TEXT NOT NULL)",
    ))
    .await
    .unwrap();

    let users = generate!("auth" @ users (5) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    let Ok(data) = users.seed(&pool).await else {
        panic!("Cannot seed the data");
    };

    println!("{data:#?}");
}
