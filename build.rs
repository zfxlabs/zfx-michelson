use std::fs;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let st = Command::new("npm").args(&["install"]).status()?;
    assert!(st.success());
    let st = Command::new("npx").args(&["webpack"]).status()?;
    assert!(st.success());
    // Delete generated LICENSE
    let lic = "./scripts/michelson_parser.bundle.js.LICENSE.txt";
    fs::remove_file(lic)?;
    return Ok(());
}
