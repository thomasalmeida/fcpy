# fcpy - File Concatenator

<p align="center">
  <img src=".github/assets/logo.png" alt="fcpy logo" width="150" />
</p>

<p align="center">
  <a href="https://crates.io/crates/fcpy"><img src="https://img.shields.io/crates/v/fcpy?color=blue&logo=rust" alt="Crates.io"></a>
  <a href="https://github.com/thomasalmeida/fcpy/releases/latest"><img src="https://img.shields.io/github/v/release/thomasalmeida/fcpy?color=blueviolet&logo=github" alt="GitHub release"></a>
  <a href="https://github.com/thomasalmeida/fcpy/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/thomasalmeida/fcpy/ci.yml?branch=main&logo=githubactions&logoColor=white" alt="CI Status"></a>
  <a href="https://github.com/thomasalmeida/fcpy/blob/main/LICENSE"><img src="https://img.shields.io/github/license/thomasalmeida/fcpy?color=green&logo=opensourceinitiative&logoColor=white" alt="License"></a>
</p>


A high-performance command-line file concatenator written in Rust, designed to safely aggregate text files while automatically ignoring binary and media files. Features clipboard integration and smart filtering.

## Features

- 📋 **Automatic clipboard copying** (wl-copy or xclip/xsel required)
- ⚡ **Intelligent binary-file detection** – prevents corrupted output
- 🛡️ **Safe path handling and permission checks**
- 🔍 **Advanced glob-pattern matching** for ignore rules
- 📁 **Recursive directory processing**
- 📦 **Cross-platform** single-binary deployment

## Installation

### Crates.io

```bash
cargo install fcpy
```

### Direct Download

```bash
# Download latest release
curl -LO https://github.com/thomasalmeida/fcpy/releases/latest/download/fcpy-linux-amd64
chmod +x fcpy-linux-amd64
sudo mv fcpy-linux-amd64 /usr/local/bin/fcpy
```

### From Source

```bash
# Clone repository
git clone https://github.com/thomasalmeida/fcpy
cd fcpy

# Build and install
make install
```

### Manual (local build)

```bash
# Build the release binary
cargo build --release

# Copy to a directory in your PATH (Linux/macOS)
sudo cp target/release/fcpy /usr/local/bin/

# Now you can run:
fcpy --help
```

## Usage

```bash
fcpy [PATHS]... [OPTIONS]
```

Options:

- `-o, --output [FILE]`
  Save output to file (default: `paste.txt` when `-o` is used without value)

- `-i, --ignore <PATTERN>...`
  Ignore files/directories matching glob patterns

- `-h, --help`
  Show help

- `-V, --version`
  Show version

## Examples

```bash
# Concatenate all .rs files, ignoring .git and target directories
fcpy src/*.rs -i .git target *.lock

# Combine multiple directories and patterns, output to log.txt
fcpy docs/ examples/ -i "**/node_modules/**" "*.zip" -o log.txt

# Process multiple files and directories
fcpy README.md LICENSE src/ -i "*.exe" "*.bin"
```

## Testing

Run unit and integration tests:

```bash
cargo test
```

## Performance Tips

Use more specific patterns for better performance:

```bash
# Good
fcpy . -i target "*.log"

# Better (specific directory exclusion)
fcpy . -i "**/node_modules/**" "*.zip"
```

## Contributing

Contributions are welcome! To propose changes:

1. Fork the repo
2. Create a feature branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -m "Add feature"`)
4. Push and open a Pull Request

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.
