#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("examples/run-passes-main.rs");
}
