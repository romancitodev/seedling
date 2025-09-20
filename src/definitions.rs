/// Marker trait indicating that a type can be used as a database column value.
///
/// Types implementing this trait can be inserted as values in columns
/// during the seeding process.
pub trait IntoValue: std::fmt::Debug + ToString {
    fn into_value(&self) -> impl ToString;
}

impl IntoValue for String {
    fn into_value(&self) -> impl ToString {
        format!("\"{self}\"")
    }
}

/// Trait defining a database table column.
///
/// Implementations of this trait must provide:
/// - A list of all available columns
/// - The column name
/// - A generated value for the column
pub trait Column: Sized {
    /// Returns a static reference to all available columns for this table.
    fn all() -> &'static [Self];
    /// Returns the column name as a string.
    fn name(&self) -> &str;
    /// Generates and returns a value for this column.
    ///
    /// This method should generate appropriate data for the column type,
    /// typically using the integrated `fake` library.
    fn value(&self) -> impl IntoValue;
}

/// Trait representing a database table with an optional schema.
///
/// # Type Parameters
/// - `S`: The schema type, defaults to `()` (no schema)
pub trait Table<S: Schema = ()> {
    /// Type representing the columns of this table.
    type Columns: Column + std::fmt::Debug;
    /// Returns the table name as a static string.
    fn table_name() -> &'static str;
}

/// Trait defining a database schema.
///
/// Schemas can be optional. If no schema is specified,
/// use the default implementation `()`.
pub trait Schema {
    fn schema_name() -> Option<&'static str>;
}

impl Schema for () {
    fn schema_name() -> Option<&'static str> {
        None
    }
}

#[macro_export]
macro_rules! generate {
    ($schema:literal @ $tname:ident ($n:literal) {
        $($key:ident: $value:expr),*
        $(,)?
    }) => {{
        const _: &'static str = $schema;
        const _: usize = $n;
        seedling_macros::procedural_generate!($schema, $tname, $n, [$(($key, $value)),*])
    }};
    ($schema:literal @ $tname:ident {
        $($key:ident: $value:expr),*
        $(,)?
    }) => {{
        const _: &'static str = $schema;
        seedling_macros::procedural_generate!($schema, $tname, 1, [$(($key, $value)),*])
    }};
    ($tname:ident ($n:literal) {
        $($key:ident: $value:expr),*
        $(,)?
    }) => {{
        const _: usize = $n;
        seedling_macros::procedural_generate!("", $tname, $n, [$(($key, $value)),*])
    }};
    ($tname:ident {
        $($key:ident: $value:expr),*
        $(,)?
    }) => {{
        seedling_macros::procedural_generate!("", $tname, 1, [$(($key, $value)),*])
    }};
}
