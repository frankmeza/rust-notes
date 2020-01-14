# Control Flow

- if expressions, and loops do much of the heavy lifting in Rust control flow.

## 1. If Expressions

These are essentially identical to standard Javascript.

- These are more like IF *statements* to me, they are executing code but still not returning a value...

```rust
fn main() {
    let number = 3;

    if number < 5 { // this line must evaluate to bool
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

### Boolean values

IF expressions in Rust are very particular that the `if` condition evaluates to `true` or `false`. There does not exist the same truthy/falsy coercion as in Javascript or even Typescript at times.

### IF / ELSE

It's just like in Javascript.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### Using IF in a LET statement, aka Ternary Expression

Because `if` in this case is an expression (returning the final value by omitting the semicolon), we can use it on the right side of a let statement:

```rust
fn main() {
    let condition = true;
    // this is the ternary equivalent in JS:
    // const number = condition ? 5 : 6

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

## 2. Loops

```rust
// this will run forever

fn main() {
    loop {
        println!("again!");
    }
}
```

### Returning from Loops

The keyword `break` works just like in Javascript.

### Conditional loops, using While Loops

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

### Using a for Loop instead of While

These look similar to Python loops through dictionary, especially with `.iter()`:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

### Range method in Rust

We iterate thusly:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```
