# Test Organization

Rust differentiates between:

- *unit* tests, and
- *integration* tests.

## Unit Tests

Supposedly, convention for testing is for the tests to go into the same __file__ as the production code, inside of a module named `tests`, and to annotate the module with `cfg(test)`. This annotation tells the compiler only to compile this when running `cargo test` and not during compiling code for development or production.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

> This code is the automatically generated test module. The attribute `cfg` stands for configuration and tells Rust that the following item should only be included given a certain configuration option. In this case, the configuration option is test, which is provided by Rust for compiling and running tests. By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test. This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].

## Testing Private Functions

Rust allows testing of private functions, like this:

```rust
// this is public
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// this is private
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    [#test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration Tests

These tests live as neighbors to the production code, in a directory called `tests`, next to `src`. Each test file is compiled as a separate crate by cargo. From the terminal, this is run with `cargo test`.

```rust
// tests/integration_test.rs

use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

### Running Tests in Specific, or only Specific Files

> We can still run a particular integration test function by specifying the test function’s name as an argument to `cargo test`. To run all the tests in a particular integration test file, use the --test argument of `cargo test` followed by the name of the file:

`$ cargo test --test integration_test`

## Submodules in Integration Tests

Ideally, integration tests are grouped into directories of related tests. This will create separate scopes, which becomes most visible when trying to share utility functions across different integration test directories.

One way to grease the wheels and make util function sharing easier is to use `tests/common` which the Rust compiler understands to contain these util functions.

```rust
// tests/common/mod.rs

pub fn setup() {
    // setup code specific to your library's tests would go here
}


// tests/integration_test.rs

use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // util fn called here from common/mod.rs
    assert_eq!(4, adder::add_two(2));
}
```

