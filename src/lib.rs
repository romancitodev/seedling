mod backend;
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

#[cfg(all(
    feature = "sqlx",
    any(
        feature = "sqlite",
        feature = "postgres",
        feature = "mysql",
        feature = "mssql"
    )
))]
impl<T: Table<S>, S: Schema, const N: usize> crate::backend::SqlxExecutor for Mock<T, S, N> {
    fn seed<DB: sqlx::Database>(
        &self,
        pool: &sqlx::Pool<DB>,
    ) -> impl Future<Output = sqlx::Result<DB::QueryResult>>
    where
        for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
        for<'c> <DB as sqlx::Database>::Arguments<'c>: sqlx::IntoArguments<'c, DB>,
    {
        use sqlx::Executor;
        let query_sql = self.prepare_insert();

        async move {
            let query = sqlx::query(&query_sql);
            println!("{query_sql}");
            pool.execute(query).await
        }
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

// pub fn run<'p, DB, T: Table<S>, S: Schema, const N: usize>(
//     pool: &'p sqlx::Pool<DB>,
//     mocks: &'p [Mock<T, S, N>],
// ) -> impl std::future::Future<Output = sqlx::Result<()>> + 'p
// where
//     DB: Database,
//     for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
//     for<'a> <DB as Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>,
//     <T as definitions::Table<S>>::Columns: 'static,
// {
//     async move {
//         for mock in mocks {
//             mock.seed(&pool).await.unwrap();
//         }
//         Ok(())
//     }
// }
