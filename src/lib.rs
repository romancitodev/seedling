//! Seedling - Database seeding library for Rust
#![doc = include_str!("../README.md")]

pub mod backend;
pub mod core;

// Re-exports from core
pub use core::{Column, IntoValue, Mock, Schema, Table};

// Backend re-exports
pub use backend::*;

// Re-export function from core
#[cfg(feature = "sqlx")]
pub use core::run;

// Re-exporting a library
pub use mockd as fake;

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
