pub mod definitions;

use definitions::*;
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
/// ```rust
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
        Self {
            table: std::marker::PhantomData::<T>,
            schema: std::marker::PhantomData::<S>,
            count: N,
        }
    }

    /// Executes the seeding process for the specified table.
    pub fn seed(&self)
    where
        <T as Table<S>>::Columns: 'static,
    {
        println!("Repeating this table this times: {}", self.count);
        let schema = S::schema_name().unwrap_or("");
        for table in T::Columns::all() {
            let value = table.value();
            println!("Running gen on {schema}.{table:#?} = {value:#?}")
        }
    }
}
