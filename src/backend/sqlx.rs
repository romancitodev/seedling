#[cfg(feature = "sqlx")]
pub trait SqlxExecutor {
    fn seed<DB: sqlx::Database>(
        &self,
        pool: &sqlx::Pool<DB>,
    ) -> impl std::future::Future<Output = sqlx::Result<DB::QueryResult>> + Send
    where
        for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
        for<'a> <DB as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>;
}
