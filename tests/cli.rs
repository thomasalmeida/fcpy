use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn cli_ignores_non_utf8_and_executables() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let out_file = dir.path().join("output.txt");

    std::fs::write(dir.path().join("test.bin"), [0u8, 159, 146, 150])?;
    std::fs::write(dir.path().join("app.exe"), "echo hello")?;
    std::fs::write(dir.path().join("ok.txt"), "Hello World")?;

    Command::cargo_bin("fcpy")?
        .arg(dir.path())
        .arg("-o")
        .arg(out_file.to_str().unwrap())
        .assert()
        .success();

    let output = std::fs::read_to_string(out_file)?;

    assert!(output.contains("ok.txt"));
    assert!(output.contains("Hello World"));
    assert!(!output.contains("test.bin"));
    assert!(!output.contains("app.exe"));

    Ok(())
}
