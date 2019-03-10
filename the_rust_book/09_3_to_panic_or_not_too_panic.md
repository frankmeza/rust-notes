# To panic! or Not to panic!

## When to call `panic!`

- calling `panic!` is like tapping out completely, you're saying that the code is unrecoverable
- by returning a `Result` value
    - you can handle the error yourself.
    - even if that means that the code is indeed unrecoverable, think POST requests with bodies that do not meet their contract.
    - `tldr;` thinking first of returning `Result` from a function that might fail is a good default choice.

## Examples, Prototype Code, and Tests

- at the prototype level, full on 100% error coverage is not ideal
    - it could obscure the code's intention, initially
    - `unwrap` and `expect` are used to leave `// TODO`s in the code
    - with tests too, in order to fail the test sooner
    - perhaps it's very conventional in Rust to return `Result`... 🤷‍♂️🤔

## Cases in Which You Have More Information Than the Compiler

- calling `unwrap` is okay to do when something is all but logically impossible

```rust
use std::net::IpAddr;

let home: IdAddr = "127.0.0.1".parse().unwrap();
```

- `parse` still returns `Result` so it must be handled one way or the other
- it's a subjective call based on your understanding of all the pieces in play

## Guidelines for Error Handling

- it's best to `panic!` if the code could be thrown into a bad state, in which
    - it's not normally expected to happen.
    - code further down relies on this info to not be in this state.
    - there's not a good way to encode this info in your used types.
- extensive error handling is helpful to others who use your API in
    - writing code that fails faster and with more accuracy.
    - calling `panic!` **IS** appropriate when calling external code, to let the user know that the error is beyond your control.
        - however, it's still better to return a `Result` than a `panic!`
- when performing operations, your code should validate values first, then `panic!` if they aren't
- operating on invalid data can result in code vulnerabilities, this is the main reason the standard library will call `panic!` when attempting an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem.

### Rust's Type System

- having oodles of error checks everywhere can be difficult to read through
- function contracts are much less verbose and clearer that several error checks
- this **MAKES** the calling code define its values, because yours will fail given invalid arguments.
- another small trick using an unsigned integer, ensuring that the value is never negative

### Creating a new type to ensure values

```rust
// the wrapper struct, aka custom type
pub struct Guess {
    value: i32,
}

impl Guess {
    // constructor function; creates a valid object
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            // first level panicking, instead of not catching the error
            // so soon, and then our code has to deal with a bad state.
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        // Guess is guaranteed to be returned
        Guess {
            value
        }
    }

    // this is typesafe, as it relies directly
    // on code ensuring the returned type is i32.
    pub fn value(&self) -> i32 {
        self.value
    }
}
```

- `Option` and `Result` are our initial views of Rust's generic types.