#![cfg(feature = "sqlx")]

use async_std::task;
use sqlx::{sqlite::SqliteConnection, Connection, Sqlite};

use base64id::Id64;

#[test]
#[cfg(feature = "sqlx")]
fn sqlx_id64_type() {
    let id = Id64::MAX;
    let mut conn = task::block_on(SqliteConnection::connect("sqlite::memory:")).unwrap();

    task::block_on(sqlx::query("CREATE TABLE sqlx (id INT PRIMARY KEY)").execute(&mut conn))
        .unwrap();

    task::block_on(
        sqlx::query("INSERT INTO sqlx VALUES (?)")
            .bind(id)
            .execute(&mut conn),
    )
    .unwrap();

    let output = task::block_on(
        sqlx::query_as::<Sqlite, Id64>("SELECT * FROM sqlx LIMIT 1").fetch_one(&mut conn),
    )
    .unwrap();

    assert_eq!(id, output);
}
