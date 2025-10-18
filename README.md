# Aurora 🎨 / Rust

![GitHub release](https://img.shields.io/badge/version-0.1.0-limegreen)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

The fastest C code syntax highlighter for the terminal, written in Rust.

> A Rust implementation of the [Aurora](https://github.com/codeAbinash/aurora) syntax highlighting engine, optimized for terminal output with beautiful colors using the One Dark Pro theme.

## ✨ Features

- **🚀 Blazing Fast**: Written in Rust for maximum performance
- **🎨 Beautiful Highlighting**: Uses the One Dark Pro theme for syntax highlighting
- **⚡ Lightweight**: Minimal dependencies, fast execution
- **📝 Terminal Output**: Direct syntax-highlighted output to your terminal
- **🎯 C Language Support**: Full support for C syntax including preprocessor directives
- **🔧 Easy to Use**: Simple command-line interface

## 🛠️ Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from source

```bash
git clone https://github.com/codeAbinash/aurora-rust.git
cd aurora-rust
cargo build --release
```

The compiled binary will be available at `target/release/aurora` (or `aurora.exe` on Windows).

### Install via Cargo

```bash
cargo install --path .
```

## 📖 Usage

### Basic Usage

```bash
aurora <filename>
```

### Example

```bash
aurora test/test.c
```

This will read the C file and print it to the terminal with beautiful syntax highlighting.

### Commands

```bash
# Display help message
aurora --help
aurora -h

# Display version
aurora --version
aurora -v

# Display about information
aurora --about
```

## 🎨 Syntax Highlighting Features

Aurora supports highlighting for:

- **Comments**: Single-line (`//`) and multi-line (`/* */`)
- **Preprocessor Directives**: `#include`, `#define`, `#ifdef`, etc.
- **Keywords**: `int`, `void`, `for`, `while`, `if`, `return`, etc.
- **Functions**: Function names and calls
- **Strings**: String literals with format specifiers
- **Numbers**: Decimal, hexadecimal (0x), octal (0), and binary (0b)
- **Operators**: Arithmetic, logical, and bitwise operators
- **Escape Sequences**: `\n`, `\t`, `\\`, etc.
- **Brackets**: Color-coded matching brackets

## 🎯 Example Output

When you run Aurora on a C file, you'll see beautifully colored output in your terminal:

```c
// Comments in gray
#include <stdio.h>  // Preprocessor in violet, header files in green

int main() {  // Keywords in violet, function names in blue
    printf("Hello, World!\n");  // Strings in green, escape sequences highlighted
    return 0;  // Keywords and numbers highlighted
}
```

## 🏗️ Project Structure

```
aurora-rust/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── lib.rs            # Library exports
│   ├── commands.rs       # Command-line argument handling
│   ├── highlighter.rs    # Syntax highlighting logic
│   ├── tokenizer.rs      # Tokenization engine
│   ├── info.rs          # Version and project info
│   └── lib/
│       ├── comments.rs          # Comment parsing
│       ├── preprocessor.rs      # Preprocessor directive parsing
│       ├── string.rs            # String literal parsing
│       ├── escape.rs            # Escape sequence handling
│       ├── number.rs            # Number literal parsing
│       └── name_or_keyword.rs   # Identifier and keyword parsing
├── test/
│   └── test.c            # Test C file
├── Cargo.toml            # Rust dependencies
└── README.md
```

## 🎨 Color Theme

Aurora uses the **One Dark Pro** theme with the following color scheme:

- **Comments**: Gray `#5c6370`
- **Keywords**: Violet `#c678dd`
- **Strings**: Green `#98c379`
- **Numbers**: Orange `#d19a66`
- **Functions**: Blue `#61afef`
- **Escape Sequences**: Blue-Green `#56b6c2`
- **Operators**: Violet `#c678dd`

## 🔧 Dependencies

- [colored](https://crates.io/crates/colored) - Terminal color library

## 🚀 Performance

Aurora is designed for speed and efficiency:

- Fast tokenization engine
- Minimal allocations
- Direct terminal output
- Zero-copy string slicing where possible

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development

```bash
# Run in development mode
cargo run -- test/test.c

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 👨‍💻 Author

**Abinash Karmakar** ([@codeAbinash](https://github.com/codeAbinash))

## 🔗 Related Projects

- [aurora](https://github.com/codeAbinash/aurora) - The original C highlighting engine for the web
- [aurora-react](https://github.com/codeAbinash/aurora-react) - React wrapper for Aurora

## 🌟 Acknowledgments

This project is a Rust implementation inspired by the original [Aurora](https://github.com/codeAbinash/aurora) project, bringing the same beautiful syntax highlighting to the terminal.

---

Made with ❤️ and Rust 🦀
