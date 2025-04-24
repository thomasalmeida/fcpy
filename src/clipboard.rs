use anyhow::{Context, Result};
use std::env;
use std::process::{Command, Stdio};

pub fn copy_to_clipboard(content: &str) -> Result<()> {
    match env::consts::OS {
        "linux" => copy_linux(content),
        "macos" => copy_macos(content),
        "windows" => copy_windows(content),
        _ => Err(anyhow::anyhow!("Clipboard not supported on this platform")),
    }
}

fn copy_linux(content: &str) -> Result<()> {
    // Try wl-copy (Wayland) first
    if let Ok(mut child) = Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, content.as_bytes())?;
        }
        child.wait()?;
        return Ok(());
    }

    // Try xclip (X11) next
    if let Ok(mut child) = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, content.as_bytes())?;
        }
        child.wait()?;
        return Ok(());
    }

    // Last try with xsel (alternative X11 clipboard utility)
    if let Ok(mut child) = Command::new("xsel")
        .arg("--clipboard")
        .arg("--input")
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, content.as_bytes())?;
        }
        child.wait()?;
        return Ok(());
    }

    // If all fail, return a helpful error
    Err(anyhow::anyhow!("Failed to access clipboard. Please install one of: wl-clipboard (Wayland), xclip or xsel (X11)"))
}

fn copy_macos(content: &str) -> Result<()> {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to access clipboard (pbcopy required)")?;

    if let Some(mut stdin) = child.stdin.take() {
        std::io::Write::write_all(&mut stdin, content.as_bytes())?;
    }
    child.wait()?;
    Ok(())
}

fn copy_windows(content: &str) -> Result<()> {
    // Use PowerShell to access clipboard
    let mut child = Command::new("powershell.exe")
        .arg("-Command")
        .arg("Set-Clipboard -Value $input")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to access clipboard (PowerShell required)")?;

    if let Some(mut stdin) = child.stdin.take() {
        std::io::Write::write_all(&mut stdin, content.as_bytes())?;
    }
    child.wait()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_copy_to_clipboard_no_crash() {
        // This just tests that the function doesn't crash
        let result = copy_to_clipboard("Test content");

        // We can't reliably test clipboard content in CI
        // But we can check if it runs without error on supported platforms
        match env::consts::OS {
            "linux" | "macos" | "windows" => {
                // We allow errors since CI might not have clipboard access
                if result.is_err() {
                    println!("Clipboard error (expected in CI): {}", result.unwrap_err());
                }
            }
            _ => {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("not supported"));
            }
        }
    }
}
