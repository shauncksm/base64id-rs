macro_rules! generate_sqlx_test_suite {
    ($lib_type:ident, $lib_type_name: ident) => {
        #[cfg(feature = "sqlx")]
        mod $lib_type_name {
            use async_std::task;
            use sqlx::{sqlite::SqliteConnection, Connection, Sqlite};

            use base64id::$lib_type;

            #[test]
            fn sqlx_type() {
                let id = $lib_type::MAX;
                let mut conn =
                    task::block_on(SqliteConnection::connect("sqlite::memory:")).unwrap();

                task::block_on(
                    sqlx::query("CREATE TABLE sqlx (id INT PRIMARY KEY)").execute(&mut conn),
                )
                .unwrap();

                task::block_on(
                    sqlx::query("INSERT INTO sqlx VALUES (?)")
                        .bind(id)
                        .execute(&mut conn),
                )
                .unwrap();

                let output = task::block_on(
                    sqlx::query_as::<Sqlite, $lib_type>("SELECT * FROM sqlx LIMIT 1")
                        .fetch_one(&mut conn),
                )
                .unwrap();

                assert_eq!(id, output);
            }
        }
    };
}

generate_sqlx_test_suite!(Id64, id64);

generate_sqlx_test_suite!(Id32, id32);
