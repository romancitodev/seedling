use std::pin::Pin;

pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[cfg(feature = "sqlx")]
pub trait SqlxExecutor<DB>
where
    DB: sqlx::Database + Send + Sync,
    for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, DB>,
{
    fn seed<'p>(
        &'p self,
        pool: &'p sqlx::Pool<DB>,
    ) -> BoxedFuture<'p, sqlx::Result<DB::QueryResult>>;
}
