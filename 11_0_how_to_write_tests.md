# How to Write Tests

Bodies of test functions do these things:

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

## The Anatomy of a Test Function

Rust uses a `derive` attribute to identify a test function, `#[test]` to report on its success.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

- `#[test]` is needed to identify which functions are test functions, as test and production code can be in the same file.
- `assert_eq!` is a testing macro function
