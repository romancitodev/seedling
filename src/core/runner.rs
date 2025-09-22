#[cfg(feature = "sqlx")]
/// Run multiple mocks in sequence against a database pool.
///
/// This function executes all provided mocks against the given database pool,
/// stopping at the first error encountered.
pub async fn run<DB>(
    pool: &sqlx::Pool<DB>,
    mocks: Vec<Box<dyn crate::backend::SqlxExecutor<DB> + Send + Sync>>,
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
