use seedling::Mock;
use seedling::definitions::{Column, IntoValue, Table};
use seedling::fake;

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
    fn all() -> &'static [Self] {
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

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    use sqlx::Executor;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    pool.execute(sqlx::query(
        "CREATE TABLE users (id TEXT PRIMARY KEY NOT NULL, username TEXT NOT NULL, email TEXT NOT NULL)",
    ))
    .await
    .unwrap();
    let users = Mock::<Users, _>::new();
    let Ok(data) = users.seed(&pool).await else {
        panic!("Cannot seed the data");
    };
    println!("{data:#?}");
    let mock = Mock::<Users, _, 5>::new();
    let data = match mock.seed(&pool).await {
        Ok(data) => data,
        Err(e) => {
            panic!("Cannot seed the data: {e:?}");
        }
    };
    println!("{data:#?}");
}

#[cfg(feature = "smol")]
#[tokio::main]
async fn main() {
    let db = sqlx::SqliteConnection::connect("sqlite::memory:")
        .await
        .unwrap();
    let users = Mock::<Users, AuthSchema>::new();
    users.seed(&db);
    let mock = Mock::<Users, _, 5>::new();
    mock.seed(&db);
}
