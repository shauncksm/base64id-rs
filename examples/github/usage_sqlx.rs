use base64id::Id64;
use sqlx::{
    sqlite::{Sqlite, SqliteConnection},
    Connection,
};
use std::str::FromStr;

fn main() {
    async_std::task::block_on(database()).unwrap();
}

async fn database() -> Result<(), sqlx::Error> {
    let id = Id64::from_str("IkoY0lQYRrI").unwrap();
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    sqlx::query("CREATE TABLE sqlx (id INT PRIMARY KEY)")
        .execute(&mut conn)
        .await?;

    sqlx::query("INSERT INTO sqlx VALUES (?)")
        .bind(id)
        .execute(&mut conn)
        .await?;

    let output = sqlx::query_as::<Sqlite, Id64>("SELECT id FROM sqlx LIMIT 1")
        .fetch_one(&mut conn)
        .await?;

    println!("{output}"); // IkoY0lQYRrI

    Ok(())
}
