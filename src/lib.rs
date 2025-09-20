pub mod definitions;

pub use definitions::*;
// Re-exporting a library
pub use mockd as fake;
#[cfg(feature = "sqlx")]
use sqlx::Database;

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

impl<T: Table<S>, S: Schema, const N: usize> Mock<T, S, N> {
    /// Creates a new instance of the data generator.
    ///
    /// The number of records to generate is specified as a constant type parameter `N`.
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
    fn prepare_insert(&self) -> String
    where
        <T as definitions::Table<S>>::Columns: 'static,
    {
        let mut sql = String::from("INSERT INTO ");
        sql.push_str(T::table_name());
        let columns = format!(
            " ({}) ",
            T::Columns::all()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>()
                .join(", ")
        );
        sql.push_str(&columns);

        sql.push_str(" VALUES ");
        let mut values = vec![];

        for _ in 0..self.count {
            let data = format!(
                "({})",
                T::Columns::all()
                    .iter()
                    .map(|c| c.value().into_value().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            values.push(data);
        }
        sql.push_str(&values.join(",\n"));

        sql
    }

    #[cfg(all(feature = "sqlx", any(feature = "sqlite")))]
    /// Executes the seeding process for the specified table.
    pub fn seed<'p, DB>(
        &'p self,
        pool: &'p sqlx::Pool<DB>,
    ) -> impl std::future::Future<Output = sqlx::Result<DB::QueryResult>> + 'p
    where
        DB: Database,
        for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
        for<'a> <DB as Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>,
        <T as definitions::Table<S>>::Columns: 'static,
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
