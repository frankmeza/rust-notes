# Controlling How Tests Are Run

`cargo test` compiles your code in test mode then runs the resulting binary. CLI args can be given to `cargo test` to augment behavior, like this:

- `cargo test --help` displays cargo test options
- `cargo test -- --help` displays options that you can use after the `--` separator`

## Running Tests in Parallel or Consecutively

- by default, tests run in parallel. But this can be configured otherwise too. You can run all tests in a single thread like this from the command line:

```
$ cargo test -- --test-threads=1
```

- make the tests idempotent, and not reliant on each other in a certain order.

## Showing Function Output

In a nutshell, `println!` will show only for failing tests. This can be configured otherwise, like this:

```
$ cargo test -- --nocapture
```

## Running a Subset of Tests by Name

Given these tests:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

By default, all tests run. Names of individual tests can be passed into `cargo test` to run a single test.

```
$ cargo test one_hundred
```

You can also pass in part of a test name that many tests share, to run several "filtered" tests at once.

```
$ cargo test add
```

This will run `add_two_and_two` and `add_three_and_two`.

## Ignoring Some Tests Unless Specifically Requested

Tests can be ignored with the decorator `#[ignore]`, like this:

```rust

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```
