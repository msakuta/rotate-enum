#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/direction.rs");
}
