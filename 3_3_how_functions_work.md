# Functions

- defined with `fn`
- functions have parameters, which make up the function signature
- when called, functions receive _arguments_, which are the concrete values actually passed into functions.

```rust
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

## Statements and Expressions

- statements do something, and then do **NOT** return a value.
- expressions **do** return something.

```rust
fn main() {
    // let y = 6 is a statement that returns nothing,
    // so this code is trying to assign a non-value to x.

    let x = (let y = 6);
}
```

- So, you cannot assign a `let` _statement_ to another variable, because there is no return value from a statement and nothing for `x` to bind to.

- Rust uses the omission of a semicolon `;` to signify that that final line of code within a function body will now return a value.

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;

        x + 1
        // no semicolon ^^ , so it returns its evaluation to
        // the parent block, making `x + 1` an expression.
    };
    // at this point, the value of y is 4

    println!("The value of y is: {}", y);
}
```

The following code will produce a compile error:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    // the semicolon makes this a statement,
    // so the fn is returning nothing.
    x + 1;
}
```

#### WHY?

- `plus_one` does not return a value, its signature is wrong
- Rust will suggest removing the semicolon in its error message.
