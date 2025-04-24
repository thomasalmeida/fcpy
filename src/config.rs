use globset::{Glob, GlobSet, GlobSetBuilder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid glob pattern: {0}")]
    InvalidGlobPattern(#[from] globset::Error),
}

pub fn build_ignore_set(patterns: &[String]) -> Result<GlobSet, ConfigError> {
    let mut builder = GlobSetBuilder::new();

    // Default ignores
    let default_ignores = &[
        "*.exe", "*.dll", "*.so", "*.dylib", "*.bin", "*.dat", "*.jar", "*.o", "*.a", "*.lib",
        "*.zip", "*.tar*", "*.gz", "*.bz2", "*.xz", "*.7z", "*.rar", "*.deb", "*.rpm", "*.jpg",
        "*.jpeg", "*.png", "*.gif", "*.bmp", "*.svg", "*.ico", "*.webp", "*.psd", "*.tiff",
        "*.mp3", "*.wav", "*.ogg", "*.m4a", "*.flac", "*.aac", "*.mp4", "*.avi", "*.mkv", "*.mov",
        "*.wmv", "*.flv", "*.webm", "*.ppt*", "*.ods", "*.odp", "*.sys", "*.dmp", "*.pak", "*.cab",
    ];

    for pattern in default_ignores {
        builder.add(Glob::new(pattern)?);
    }

    for user_pattern in patterns {
        // Trim whitespace and remove any trailing '/' so that "target" and "target/" are equivalent.
        let clean_pattern = user_pattern.trim().trim_end_matches('/');

        if !clean_pattern.is_empty() {
            // Add the pattern as provided by the user.
            builder.add(Glob::new(clean_pattern)?);

            // If the pattern does not contain a slash, add recursive variants.
            if !clean_pattern.contains('/') {
                // This variant matches the pattern in any subdirectory.
                builder.add(Glob::new(&format!("**/{}", clean_pattern))?);
                // For literal patterns (without wildcards), also ignore directories with the same name.
                if !clean_pattern.contains('*')
                    && !clean_pattern.contains('?')
                    && !clean_pattern.contains('[')
                {
                    builder.add(Glob::new(&format!("**/{}/**", clean_pattern))?);
                }
            }
        }
    }

    Ok(builder.build()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ignore_set() -> Result<(), ConfigError> {
        // Test default ignores
        let default_set = build_ignore_set(&[])?;
        assert!(default_set.is_match("file.exe"));
        assert!(default_set.is_match("path/to/file.jpg"));
        assert!(!default_set.is_match("file.txt"));

        // Test custom ignores
        let custom_set = build_ignore_set(&["*.txt".to_string()])?;
        assert!(custom_set.is_match("file.txt"));
        assert!(custom_set.is_match("path/to/file.txt"));
        assert!(custom_set.is_match("file.exe")); // Default pattern

        // Test directory ignores
        let dir_set = build_ignore_set(&["node_modules".to_string()])?;
        assert!(dir_set.is_match("node_modules"));
        assert!(dir_set.is_match("path/to/node_modules"));
        assert!(dir_set.is_match("node_modules/file.txt"));

        Ok(())
    }

    #[test]
    fn test_build_ignore_set_invalid_pattern() {
        let result = build_ignore_set(&["[".to_string()]);
        assert!(result.is_err());
    }
}
