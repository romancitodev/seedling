# 🌱 Seedling

A modern, type-safe database seeding library for Rust that makes generating test data elegant and effortless.

[![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## ✨ Features

- **Type-Safe**: Leverages Rust's type system to prevent runtime errors
- **Macro-Driven**: Clean, declarative syntax using procedural macros
- **SQLx Integration**: Native support for database operations with async/await
- **Schema Support**: Optional schema definitions for better organization
- **Fake Data Generation**: Built-in integration with the `mockd` library
- **Batch Operations**: Run multiple mocks together with `seedling::run()`

## 🚀 Quick Start

Add Seedling to your `Cargo.toml`:

```toml
[dependencies]
seedling = "0.1.0"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"] }
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## 📖 Usage

### Using the Declarative Macro

The easiest way to seed your database is with the `generate!` macro:

```rust
use seedling::{generate, fake};
use sqlx::Executor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    
    // Create your table
    pool.execute(sqlx::query(
        "CREATE TABLE users (
            id TEXT PRIMARY KEY, 
            username TEXT NOT NULL, 
            email TEXT NOT NULL
        )"
    )).await?;

    // Generate 5 users with fake data
    let users = generate!("auth" @ users (5) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    // Seed the database
    users.seed(&pool).await?;
    
    Ok(())
}
```

### Manual Implementation

For more control, implement the traits manually:

```rust
use seedling::{Mock, SqlxExecutor, definitions::{Column, IntoValue, Table}};
use seedling::fake;

struct Users;

impl Table for Users {
    type Columns = UserColumns;
    
    fn table_name() -> &'static str {
        "users"
    }
}

#[derive(Debug)]
enum UserColumns {
    Id,
    Username,
    Email,
}

impl Column for UserColumns {
    fn all<'c>() -> &'c [Self] {
        &[UserColumns::Id, UserColumns::Username, UserColumns::Email]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Username => "username", 
            Self::Email => "email",
        }
    }

    fn value(&self) -> impl IntoValue {
        match &self {
            Self::Id => fake::unique::uuid_v4(),
            Self::Username => fake::name::first(),
            Self::Email => fake::contact::email(),
        }
    }
}

// Create individual mocks
let mock = Mock::<Users>::new(); // Generates 1 record
let batch_mock = Mock::<Users, _, 5>::new(); // Generates 5 records

// Seed individually
mock.seed(&pool).await?;

// Or run multiple mocks together
seedling::run(&pool, vec![Box::new(mock), Box::new(batch_mock)]).await?;
```

## 🛠️ Available Features

| Feature | Description | Default |
|---------|-------------|---------|
| `sqlx-tokio` | SQLx with Tokio runtime support | ✅ |
| `sqlite` | SQLite database support | ✅ |
| `postgres` | PostgreSQL support (planned) | ❌ |
| `mysql` | MySQL support (planned) | ❌ |
| `mssql` | Microsoft SQL Server support (planned) | ❌ |
| `libsql` | LibSQL backend support (planned) | ❌ |

## 🎯 Macro Syntax

The `generate!` macro supports several patterns:

```rust
// With schema and count
generate!("schema_name" @ table_name (count) { 
    column: value,
    // ...
});

// With schema, single record
generate!("schema_name" @ table_name { 
    column: value,
    // ...
});

// Without schema, with count
generate!(table_name (count) { 
    column: value,
    // ...
});

// Without schema, single record
generate!(table_name { 
    column: value,
    // ...
});
```

## 🔧 Core Traits

### `Table<S>`
Defines a database table with an optional schema:
```rust
trait Table<S: Schema = ()> {
    type Columns: Column + std::fmt::Debug;
    fn table_name() -> &'static str;
}
```

### `Column`
Represents table columns:
```rust
trait Column: Sized {
    fn all() -> &'static [Self];
    fn name(&self) -> &str;
    fn value(&self) -> impl IntoValue;
}
```

### `IntoValue`
Converts values to database-insertable format:
```rust
trait IntoValue: std::fmt::Debug + ToString {
    fn as_value(&self) -> impl ToString;
}
```

### `Schema`
Defines database schemas (optional):
```rust
trait Schema {
    fn schema_name() -> Option<&'static str>;
}
```

## 📝 Examples

Check out the [`examples/`](examples/) directory for complete working examples:

- [`users.rs`](examples/users.rs) - Manual trait implementation with batch operations
- [`user_macro.rs`](examples/user_macro.rs) - Macro-based approach with schema support

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🗺️ Roadmap

### Planned Database Support
- **PostgreSQL** - Full PostgreSQL support with SQLx
- **MySQL** - MySQL/MariaDB support with SQLx
- **Microsoft SQL Server** - MSSQL support with SQLx

### Planned Backend Support
- **LibSQL** - Native LibSQL backend integration
- **rusqlite** - Direct SQLite support without SQLx

## 🙏 Acknowledgments

- Built with [SQLx](https://github.com/launchbadge/sqlx) for database connectivity
- Uses [mockd](https://crates.io/crates/mockd) for fake data generation
- Inspired by modern database seeding tools across different ecosystems
