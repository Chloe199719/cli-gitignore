# cli-gitignore

## Description

`cli-gitignore` is a simple ignore file generator written in Rust.

## Author

[Chloe](mailto:chloevision97@gmail.com)

## License

[MIT](LICENSE)

## Repository

[https://github.com/Chloe199719/cli-gitignore](https://github.com/Chloe199719/cli-gitignore)

## Version

0.1.4

## Usage

The tool provides several command-line options:

- `--help, -h`: Show the help message.
- `--version, -v`: Show the version.
- `--language, -l`: Specify a Programming Language (typescript, javascript, rust ,append). (append is used to not use a language template and append custom commands only)
- `--remove, -r`: Remove the current .gitignore file if it exists and generate a new one.
- `--custom, -c`: Add custom entries to the .gitignore file. Example: `-c .idea .vscode`.

If no language is specified, the tool will prompt the user to enter a language. The tool currently supports generating `.gitignore` files for Typescript, Javascript, and Rust.

## Installation

### From Source

1. Clone the repository:

```bash
https://github.com/Chloe199719/cli-gitignore
```

2. Build the project:

```bash
cargo build --release
```

3. Run the executable:

```bash
./target/release/cli-gitignore
```

### From Crates.io

1. Install the tool:

```bash
cargo install cli-gitignore
```

2. Run the executable:

```bash
cli-gitignore
```

## Examples

### Generate a .gitignore file for Typescript

```bash
cli-gitignore -l typescript
```

### Generate a .gitignore file for Rust

```bash
cli-gitignore -l rust
```

### Generate a .gitignore file for Rust with custom entries

```bash
cli-gitignore -l rust -c .idea .vscode
```

### Generate a .gitignore file in Append mode

```bash
cli-gitignore -l append -c .idea .vscode
```
