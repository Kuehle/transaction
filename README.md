# Transactions

A small demo for in memory transaction procession based on an input `.csv`.

## Testing

`cargo run input.csv > output.csv` will produce an `output.csv`. You can use `cargo run input.csv > output.csv && diff output-baseline.csv output.csv` to compare against a baseline output.

> TODO: add unit tests for business logic in `src/handle.rs`
