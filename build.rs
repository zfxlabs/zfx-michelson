use std::process::Command;
use std::fs;

fn main() -> std::io::Result<()>{
    let st = Command::new("npm").args(&["install"]).status()?;
    assert!(st.success());
    let st = Command::new("npx").args(&["webpack"]).status()?;
    assert!(st.success());
    // Delete generated LICENSE
    let lic = "michelson_parser.bundle.js.LICENSE.txt";
    fs::remove_file(lic)?;
    return Ok(());
}