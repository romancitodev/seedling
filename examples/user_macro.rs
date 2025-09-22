use seedling::{SqlxExecutor, fake, generate};
use sqlx::Pool;
use sqlx::Sqlite;

#[tokio::main]
async fn main() {
    use sqlx::Executor;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(sqlx::query(
        "CREATE TABLE users (id TEXT PRIMARY KEY NOT NULL, username TEXT NOT NULL, email TEXT NOT NULL)",
    ))
    .await
    .unwrap();

    run_by_parts(&pool).await;
    run_all(&pool).await;
}

async fn run_by_parts(pool: &Pool<Sqlite>) {
    let users = generate!("auth" @ users (5) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    let Ok(data) = users.seed(pool).await else {
        panic!("Cannot seed the data");
    };
    println!("{data:#?}");
}

async fn run_all(pool: &Pool<Sqlite>) {
    let users = generate!("auth" @ users (5) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    let mock = generate!("auth" @ users (10) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    seedling::run(pool, vec![Box::new(users), Box::new(mock)])
        .await
        .unwrap();
}
