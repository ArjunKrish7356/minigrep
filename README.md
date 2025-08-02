# Minigrep

A simple grep-like text search utility implemented in Rust.

## Features

- **Text search**: Search for a query string in a file and display matching lines
- **Case-sensitive search** (default): Exact case matching
- **Case-insensitive search**: Set `CASE_INSENSITIVE=1` environment variable
- **Line numbers**: Set `LINE_NUMBERS=1` to show line numbers with results
- **Count matches**: Set `COUNT_ONLY=1` to show only the count of matching lines

## Usage

Basic usage:
```bash
cargo run -- <query> <filename>
```

### Examples

Search for "road" in poem.txt:
```bash
cargo run -- "road" poem.txt
```

Case-insensitive search:
```bash
CASE_INSENSITIVE=1 cargo run -- "ROAD" poem.txt
```

Show line numbers:
```bash
LINE_NUMBERS=1 cargo run -- "road" poem.txt
```

Count matches only:
```bash
COUNT_ONLY=1 cargo run -- "road" poem.txt
```

Combine features:
```bash
CASE_INSENSITIVE=1 LINE_NUMBERS=1 cargo run -- "ROAD" poem.txt
```

## Testing

Run the test suite:
```bash
cargo test
```