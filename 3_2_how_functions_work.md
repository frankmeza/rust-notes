# Functions

-   defined with `fn`
-   functions have parameters, which make up the function signature
-   when called, functions receive _arguments_, which are concrete values passed into functions

```rust
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

## Statements and Expressions

-   statements do something, and then do not return a value
-   expressions do return something

```rust
fn main() {
    let x = (let y = 6);
}
```

-   so, you cannot assign a `let` _statement_ to another variable, because there is no return value from a statement and nothing for `x` to bind to

-   Rust uses the omission of a semicolon `;` to signify that that final line of code within a function body will now return a value

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // no semicolon, so it returns its evaluation to the parent block
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
    x + 1;
}
```

#### WHY?

-   `plus_one` does not return a value, its signature is wrong
-   Rust will suggest removing the semicolon in its error message
