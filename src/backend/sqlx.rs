#[cfg(feature = "sqlx")]
#[async_trait::async_trait]
pub trait SqlxExecutor<DB>
where
    DB: sqlx::Database + Send + Sync,
    for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>,
{
    async fn seed(&self, pool: &sqlx::Pool<DB>) -> sqlx::Result<DB::QueryResult>;
}
