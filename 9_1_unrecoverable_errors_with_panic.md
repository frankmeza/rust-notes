# Unrecoverable Errors with `panic!`

- `panic!` is expensive and sometimes you just want the program to quit as is. You can do this with `panic = 'abort'` in `Cargo.toml`

```rust
[profile.release]
panic = 'abort'
```

## Using a panic! Backtrace

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

- in the above, Rust will panic.
- in order to protect against unexpected behavior, Rust will not perform a "buffer overread" and will instead stop execution