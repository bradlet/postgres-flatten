#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/run-passes-main.rs");
}