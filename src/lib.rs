mod backend;
#[cfg(feature = "rusqlite")]
use std::sync::Arc;

pub use backend::*;
pub mod definitions;

pub use definitions::*;
// Re-exporting a library
pub use mockd as fake;

/// Main structure for generating and seeding test data into tables.
///
/// # Type Parameters
/// - `T`: The table type implementing `Table<S>`
/// - `S`: The schema type, defaults to `()`
/// - `N`: Number of records to generate, defaults to 1
///
/// # Example
/// ```rust,no-run
/// // Create a mock that generates 10 records
/// let mock = Mock::<MyTable, (), 10>::new();
/// mock.seed();
/// ```
pub struct Mock<T: Table<S>, S: Schema = (), const N: usize = 1> {
    table: std::marker::PhantomData<T>,
    schema: std::marker::PhantomData<S>,
    count: usize,
}

impl<T: Table<S>, S: Schema, const N: usize> Default for Mock<T, S, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "rusqlite")]
impl<T: Table<S>, S: Schema, const N: usize> RusqliteExecutor for Mock<T, S, N> {
    /// Because of `rusqlite::Connection` isn't `Send`, we need to wrap it in an `Arc`
    ///
    /// So we can use it multiple times or even across threads.
    fn seed(&self, pool: Arc<rusqlite::Connection>) -> rusqlite::Result<usize> {
        let sql = {
            let mut sql = String::from("INSERT INTO ");
            sql.push_str(T::table_name());
            let columns = T::Columns::all();
            let names = format!(
                " ({}) ",
                columns
                    .iter()
                    .map(definitions::Column::name)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            sql.push_str(&names);

            sql.push_str(" VALUES ");
            let mut values = vec![];

            for _ in 0..self.count {
                let data = format!(
                    "({})",
                    columns
                        .iter()
                        .map(|c| c.value().as_value().to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                );
                values.push(data);
            }
            sql.push_str(&values.join(",\n"));

            sql
        };

        pool.execute(&sql, ())
    }
}

#[cfg(all(
    feature = "sqlx",
    any(
        feature = "sqlite",
        feature = "postgres",
        feature = "mysql",
        feature = "mssql"
    )
))]
impl<T: Table<S>, S: Schema, const N: usize, DB: sqlx::Database + Send + Sync>
    crate::backend::SqlxExecutor<DB> for Mock<T, S, N>
where
    T: Table<S> + Send + Sync,
    S: Schema + Send + Sync,
    DB: sqlx::Database + Send + Sync,
    for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>,
{
    fn seed<'p>(&self, pool: &'p sqlx::Pool<DB>) -> BoxedFuture<'p, sqlx::Result<DB::QueryResult>> {
        use sqlx::Executor;
        let query_sql = self.prepare_insert();

        Box::pin(async move {
            let query = sqlx::query(&query_sql);
            println!("{query_sql}");
            pool.execute(query).await
        })
    }
}

impl<T: Table<S>, S: Schema, const N: usize> Mock<T, S, N> {
    /// Creates a new instance of the data generator.
    ///
    /// The number of records to generate is specified as a constant type parameter `N`.
    #[must_use]
    pub fn new() -> Self {
        const {
            assert!(N >= 1);
        }
        Self {
            table: std::marker::PhantomData::<T>,
            schema: std::marker::PhantomData::<S>,
            count: N,
        }
    }

    #[cfg(feature = "sqlite")]
    fn prepare_insert(&self) -> String {
        let mut sql = String::from("INSERT INTO ");
        sql.push_str(T::table_name());
        let columns = T::Columns::all();
        let names = format!(
            " ({}) ",
            columns
                .iter()
                .map(definitions::Column::name)
                .collect::<Vec<_>>()
                .join(", ")
        );
        sql.push_str(&names);

        sql.push_str(" VALUES ");
        let mut values = vec![];

        for _ in 0..self.count {
            let data = format!(
                "({})",
                columns
                    .iter()
                    .map(|c| c.value().as_value().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            values.push(data);
        }
        sql.push_str(&values.join(",\n"));

        sql
    }
}

#[cfg(feature = "sqlx")]
pub async fn run<DB>(
    pool: &sqlx::Pool<DB>,
    mocks: Vec<Box<dyn SqlxExecutor<DB> + Send + Sync>>,
) -> sqlx::Result<()>
where
    DB: sqlx::Database + Send + Sync,
    for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>,
{
    for mock in mocks {
        mock.seed(pool).await?;
    }
    Ok(())
}
