#[test]
fn test_proc_macro_compilation() {
    let t = trybuild::TestCases::new();
    t.pass("examples/run-passes-main.rs");
}

#[cfg(test)]
mod runtime_tests {
    use postgres::{Client, NoTls};
    use postgres_flatten::{
        flattened::FromFlattenedSql, flattened::ToFlattenedSql, FromFlattenedSql, ToFlattenedSql,
    };

    #[allow(dead_code)]
    #[derive(ToFlattenedSql, FromFlattenedSql)]
    struct Cat {
        name: String,
        age: i8,
        color: i8,
        friendliness: i8,
    }

    #[test]
    #[should_panic]
    #[ignore] // Note: This integration test requires a local postgres instance. Uncomment to run when an instance is running.
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
