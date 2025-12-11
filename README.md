# Advent of Code 2025

Advent of Code 2025 solutions written in Rust.

## Running Solutions

```bash
cargo run --profile opt -p day_1 path/to/input.txt
```

## Adding a New Day

1. Create a new directory: `mkdir day_N`
2. Add a `Cargo.toml`:

   ```toml
   [package]
   name = "day_N"
   version.workspace = true
   edition.workspace = true

   [dependencies]
   common = { path = "../common" }

   [[bin]]
   name = "day_N"
   path = "src/main.rs"
   ```

3. Add `"day_N"` to the `members` array in the root `Cargo.toml`
