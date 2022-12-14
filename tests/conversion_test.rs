/// Execute a NodeJS script and check its exit status
///
/// Fails if `node` is not in the `PATH`
#[allow(unused)]
fn js_test_runner(js_file: &str) {
    use std::process::Command;
    // Looks like `cargo test` always executes in the project root directory
    let st = Command::new("node").args(&[js_file]).status().unwrap();
    assert!(st.success());
}

/// Run `npm test`
fn npm_test_runner() {
    use std::process::Command;
    // Looks like `cargo test` always executes in the project root directory
    let st = Command::new("npm").arg("test").status().unwrap();
    assert!(st.success());
}

#[test]
fn run_json_conversion_test() {
    // js_test_runner("./tests/conversion-test.js");
    npm_test_runner();
}
