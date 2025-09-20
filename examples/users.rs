use seedling::Mock;
use seedling::SqlxExecutor;
use seedling::definitions::{Column, IntoValue, Table};
use seedling::fake;
use sqlx::Pool;
use sqlx::Sqlite;

struct Users;

impl Table for Users {
    type Columns = UserColumns;

    fn table_name() -> &'static str {
        "users"
    }
}

#[derive(Debug)]
enum UserColumns {
    Id,
    Username,
    Email,
}

impl Column for UserColumns {
    fn all<'c>() -> &'c [Self] {
        &[UserColumns::Id, UserColumns::Username, UserColumns::Email]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Username => "username",
            Self::Email => "email",
        }
    }

    fn value(&self) -> impl IntoValue {
        match &self {
            Self::Id => fake::unique::uuid_v4(),
            Self::Username => fake::name::first(),
            Self::Email => fake::contact::email(),
        }
    }
}

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
    let users = Mock::<Users>::new();
    let data = match users.seed(pool).await {
        Ok(data) => data,
        Err(e) => {
            panic!("Cannot seed the data: {e:?}");
        }
    };
    println!("{data:#?}");
    let mock = Mock::<Users, _, 5>::new();
    let data = match mock.seed(pool).await {
        Ok(data) => data,
        Err(e) => {
            panic!("Cannot seed the data: {e:?}");
        }
    };
    println!("{data:#?}");
}

async fn run_all(pool: &Pool<Sqlite>) {
    let users = Mock::<Users>::new();
    let mock = Mock::<Users, _, 5>::new();
    seedling::run(pool, vec![Box::new(users), Box::new(mock)])
        .await
        .unwrap();
}
