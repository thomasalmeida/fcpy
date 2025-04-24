use assert_cmd::Command;
use tempfile::TempDir;

#[test]
fn test_basic_functionality() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let out_file = dir.path().join("output.txt");

    std::fs::write(dir.path().join("test1.txt"), "Hello")?;
    std::fs::write(dir.path().join("test2.txt"), "World")?;

    Command::cargo_bin("fcpy")?
        .arg(dir.path())
        .arg("-o")
        .arg(out_file.to_str().unwrap())
        .assert()
        .success();

    let output = std::fs::read_to_string(out_file)?;
    assert!(output.contains("test1.txt"));
    assert!(output.contains("test2.txt"));
    Ok(())
}

#[test]
fn test_ignore_patterns() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let out_file = dir.path().join("output.txt");

    std::fs::write(dir.path().join("file.log"), "Log")?;
    std::fs::write(dir.path().join("file.txt"), "Text")?;

    Command::cargo_bin("fcpy")?
        .arg(dir.path())
        .arg("-i")
        .arg("*.log")
        .arg("-o")
        .arg(out_file.to_str().unwrap())
        .assert()
        .success();

    let output = std::fs::read_to_string(out_file)?;
    assert!(!output.contains("file.log"));
    assert!(output.contains("file.txt"));
    Ok(())
}

#[test]
fn test_binary_files_ignored() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let out_file = dir.path().join("output.txt");

    std::fs::write(dir.path().join("test.bin"), [0u8, 159, 146, 150])?;
    std::fs::write(dir.path().join("test.txt"), "Text content")?;

    Command::cargo_bin("fcpy")?
        .arg(dir.path())
        .arg("-o")
        .arg(out_file.to_str().unwrap())
        .assert()
        .success();

    let output = std::fs::read_to_string(out_file)?;
    assert!(output.contains("test.txt"));
    assert!(!output.contains("test.bin"));
    Ok(())
}

#[test]
fn test_invalid_path() -> anyhow::Result<()> {
    Command::cargo_bin("fcpy")?
        .arg("/non/existent/path")
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_empty_directory() -> anyhow::Result<()> {
    let dir = TempDir::new()?;
    let out_file = dir.path().join("output.txt");

    Command::cargo_bin("fcpy")?
        .arg(dir.path())
        .arg("-o")
        .arg(out_file.to_str().unwrap())
        .assert()
        .failure();

    Ok(())
}
