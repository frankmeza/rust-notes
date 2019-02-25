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

- `#[test]` is needed to identify which functions are test functions, as test, test helpers, and production code can be in the same file.
- `assert_eq!` is a testing macro function
- each test is run in a new thread; when the main thread sees that a test thread has died, the test is marked failed.

### Let's See a Failure

```rust
#[test]
fn another() {
    panic!("Make this test fail");
}
```

### Checking results with `assert!` macro

- used with checking for a boolean `true`
- if value evaluates to `false`, `assert!` calls `panic!`
- most easily seen/used with fns that return a boolean value

### Scope of Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

> Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this tests module.

> We’ve named our test larger_can_hold_smaller, and we’ve created the two Rectangle instances that we need. Then we called the assert! macro and passed it the result of calling larger.can_hold(&smaller). This expression is supposed to return true, so our test should pass.

## Testing Equality with the assert_eq! and assert_ne! Macros

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // essentially, return 4 == add_two(2)
        assert_eq!(4, add_two(2));
    }
}
```

- `assert_ne!` is semantically asserting that the two params are unequal

## Adding Custom Failure Messages

> You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros. Any arguments specified after the one required argument to assert! or the two required arguments to assert_eq! and assert_ne! are passed along to the format! macro (discussed in Chapter 8 in the “Concatenation with the + Operator or the format! Macro” section), so you can pass a format string that contains {} placeholders and values to go in those placeholders. Custom messages are useful to document what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // alternate version with custom error message

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}
```

## Checking for a fn to Panic with `should_panic`

This demarcation lets the compiler know to panic! should this fn fail

```rust
#[test]
#[should_panic]
```


```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
```

- the downside of `should_panic!` is that *any* panic will pass, causing false positives
- this can be mitigated, like this:

```rust
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

## Using `Result<T, E>` in Tests

- tests can be written that use `Result<T, E>` as well
- So far, I like this idea the best. But `assert` comparators are the easiest to wrap my head around.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
