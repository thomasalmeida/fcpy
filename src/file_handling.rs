use anyhow::{Context, Result};
use encoding_rs::UTF_8;
use std::{
    fs::File,
    io::Read,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;

/// Normalizes a path by removing any current-directory components (i.e. ".")
fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for comp in path.components() {
        if let Component::CurDir = comp {
            continue; // skip "." components
        }
        normalized.push(comp.as_os_str());
    }
    normalized
}

/// Processes files in the given paths, ignoring files or directories matching the ignore patterns.
/// It concatenates the contents of text files.
pub fn process_files(paths: &[String], ignore_patterns: &[String]) -> Result<String> {
    let mut output = String::new();
    let globset = crate::config::build_ignore_set(ignore_patterns)?;

    for path in paths {
        let base_path = Path::new(path);
        if !base_path.exists() {
            return Err(anyhow::anyhow!("Path not found: {}", path));
        }

        for entry in WalkDir::new(base_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            // Get a relative path to the base and normalize it
            let relative_path = entry_path.strip_prefix(base_path).unwrap_or(entry_path);
            let normalized_relative = normalize_path(relative_path);

            // Skip ignored paths
            if globset.is_match(&normalized_relative) {
                continue;
            }

            // Only process files recognized as text
            if entry.file_type().is_file() && is_readable_text(entry_path)? {
                if let Ok(content) = std::fs::read(entry_path) {
                    let (text, _, _) = UTF_8.decode(&content);
                    output.push_str(&format!("==== {} ====\n{}\n\n", entry_path.display(), text));
                }
            }
        }
    }

    if output.is_empty() {
        Err(anyhow::anyhow!("No readable text files found"))
    } else {
        Ok(output)
    }
}

/// Checks if a file is readable as text.
/// Returns Ok(false) for binary or non-UTF8 content.
fn is_readable_text(path: &Path) -> Result<bool> {
    let metadata = path.metadata()?;
    if metadata.len() == 0 {
        return Ok(false);
    }

    let mut buffer = [0; 1024];
    let mut file = File::open(path).context("Failed to open file")?;
    let bytes_read = file.read(&mut buffer).context("Failed to read file")?;
    if bytes_read == 0 || buffer[..bytes_read].contains(&0) {
        return Ok(false);
    }

    let sample = &buffer[..bytes_read.min(512)];
    let text = match std::str::from_utf8(sample) {
        Ok(t) => t,
        Err(_) => return Ok(false),
    };
    let printable = text
        .chars()
        .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .count();
    Ok((printable as f32 / text.len() as f32) > 0.85)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_normalize_path() {
        let path = Path::new("./Cargo.lock");
        assert_eq!(normalize_path(path), PathBuf::from("Cargo.lock"));

        let path2 = Path::new("./src/./main.rs");
        assert_eq!(normalize_path(path2), PathBuf::from("src/main.rs"));
    }

    #[test]
    fn test_is_readable_text() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "This is a text file")?;
        assert!(is_readable_text(temp_file.path())?);
        Ok(())
    }

    #[test]
    fn test_non_utf8_file_ignored() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        // Write invalid UTF-8 sequence
        temp_file.write_all(&[0xFF, 0xFE, 0xFD])?;
        assert!(!is_readable_text(temp_file.path())?);
        Ok(())
    }

    #[test]
    fn test_process_files_ignore() -> Result<()> {
        // Create a temporary directory structure
        let dir = tempdir()?;
        let base = dir.path();

        // Create files and directories:
        std::fs::create_dir(base.join("target"))?;
        std::fs::write(base.join("Cargo.lock"), "Lock file content")?;
        std::fs::write(base.join("file.txt"), "Some text")?;
        std::fs::write(base.join("target").join("ignored.txt"), "Should be ignored")?;

        // Process files using the base directory and ignore patterns "Cargo.lock" and "target"
        let result = process_files(
            &[base.to_string_lossy().to_string()],
            &["Cargo.lock".to_string(), "target".to_string()],
        )?;

        // The result should include "file.txt" but not "Cargo.lock" nor "ignored.txt"
        assert!(result.contains("file.txt"));
        assert!(!result.contains("Cargo.lock"));
        assert!(!result.contains("ignored.txt"));
        Ok(())
    }

    #[test]
    fn test_default_ignore_executable() -> Result<()> {
        let dir = tempdir()?;
        let exe_path = dir.path().join("run.exe");
        std::fs::write(&exe_path, "print(\"Hi\")")?;

        // No custom ignore patterns -> default ignores should skip *.exe
        let output =
            process_files(&[dir.path().to_string_lossy().to_string()], &[]).unwrap_or_default();
        assert!(!output.contains("run.exe"));
        Ok(())
    }
}
