#[test]
fn test_proc_macro_compilation() {
    let t = trybuild::TestCases::new();
    // Re-using example for base test-case:
    t.pass("examples/example.rs");
    // All other UI tests:
    t.compile_fail("tests/ui/compile-fails-unsupported-type.rs");
}

#[cfg(test)]
mod runtime_tests {
    use postgres::{Client, NoTls};
    use postgres_flatten::{flattened::FromFlattenedSql, FromFlattenedSql};

    #[allow(dead_code)]
    #[derive(FromFlattenedSql)]
    struct Cat {
        name: String,
        age: i8,
    }

    #[test]
    #[should_panic(
        expected = "Type mismatch: cannot convert between the Rust type `i8` and the Postgres type `int4`"
    )]
    // #[ignore] // Note: This integration test requires a local postgres instance. Uncomment to run when an instance is running.
    fn test_panics_on_type_mismatch() {
        let mut client = Client::connect(
            "host=localhost dbname=postgres port=32768 user=brad password=password",
            NoTls,
        )
        .unwrap();
        for row in client.query("SELECT * FROM cats;", &[]).unwrap() {
            let _cat = Cat::from_flattened_row(row);
        }
    }
}
