use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "fcpy",
    version,
    about = "Fast command-line file concatenator with smart filtering, recursive scanning, and clipboard integration",
    long_about = None
)]
pub struct CliArgs {
    /// Paths to files or directories (shell glob expansion is supported)
    #[arg(required = true)]
    pub paths: Vec<String>,

    /// Save output to file (default: paste.txt when flag is used without value)
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_missing_value = "paste.txt",
        num_args = 0..=1
    )]
    pub output: Option<String>,

    /// Ignore patterns (glob patterns to exclude files/directories)
    #[arg(short, long, value_name = "PATTERN", num_args = 1..)]
    pub ignore: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_args() {
        let args = CliArgs::parse_from(["fcpy", "path"]);
        assert_eq!(args.paths, vec!["path"]);
        assert!(args.output.is_none());
        assert!(args.ignore.is_empty());
    }

    #[test]
    fn test_multiple_paths() {
        let args = CliArgs::parse_from(["fcpy", "path1", "path2"]);
        assert_eq!(args.paths, vec!["path1", "path2"]);
    }

    #[test]
    fn test_output_flag_with_value() {
        let args = CliArgs::parse_from(["fcpy", "path", "-o", "output.txt"]);
        assert_eq!(args.output, Some("output.txt".to_string()));
    }

    #[test]
    fn test_output_flag_without_value() {
        let args = CliArgs::parse_from(["fcpy", "path", "-o"]);
        assert_eq!(args.output, Some("paste.txt".to_string()));
    }

    #[test]
    fn test_ignore_patterns() {
        let args = CliArgs::parse_from(["fcpy", "path", "-i", "*.log", "*.tmp"]);
        assert_eq!(args.ignore, vec!["*.log", "*.tmp"]);
    }
}
