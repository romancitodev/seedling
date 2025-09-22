#[cfg(feature = "rusqlite")]
use std::sync::Arc;

#[cfg(feature = "rusqlite")]
pub trait RusqliteExecutor {
    /// Seeds the database with initial data.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    fn seed(&self, pool: Arc<rusqlite::Connection>) -> rusqlite::Result<usize>;
}
