#![allow(clippy::arc_with_non_send_sync)]

use rusqlite::Connection;
use seedling::{RusqliteExecutor, fake, generate};
use std::sync::Arc;

#[cfg(feature = "rusqlite")]
fn main() {
    let pool = Arc::new(rusqlite::Connection::open_in_memory().unwrap());
    pool.execute(
        "CREATE TABLE users (id TEXT PRIMARY KEY NOT NULL, username TEXT NOT NULL, email TEXT NOT NULL)",
        ()
    )
    .unwrap();

    run_by_parts(pool.clone());
    run_all(pool);
}

fn run_by_parts(pool: Arc<Connection>) {
    let users = generate!("auth" @ users (5) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    let Ok(data) = users.seed(pool) else {
        panic!("Cannot seed the data");
    };
    println!("{data:#?}");
}

fn run_all(pool: Arc<Connection>) {
    let users = generate!("auth" @ users (5) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    let mock = generate!("auth" @ users (10) {
        id: fake::unique::uuid_v4(),
        username: fake::name::first(),
        email: fake::contact::email()
    });

    let _ = users.seed(pool.clone());
    let _ = mock.seed(pool);
}
