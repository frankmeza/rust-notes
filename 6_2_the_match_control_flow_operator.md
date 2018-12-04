# The `match` Control Flow Operator

- acts as a coin sorting, plinko machine that handles every single case

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // these are called arms:
        // a pattern to match,
        // then code to execute.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // the code can be a block, with {}
        Coin::Quarter => {
            println!("Lucky quarter!");
            25
        },
    }
}
```

- this is almost identical to TS/JS `switch` statement

## Patterns that Bind to Values

```rust
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

The `match` operator and `enum` can work together even more deeply:

```rust

#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // how to make use of variable attached to an enum
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

```

## Matching with `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);

// if 5 is Some, i.e. 5 != Null
let six = plus_one(five);
let none = plus_one(None);
```

- `match` can be used with `Option<T>` to handle Null values

### A common pattern in Rust: `match`, `enum`, `enum`-bound data, code execution

- a match function compares enums on their bound data, then executes a code block

## Matches Are Exhaustive

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(i + 1)
    }
}
```

- `None` was not covered with a code block in the `match` operator

## The `_` Placeholder

- this acts just as a `default` within a TS/JS `switch` statement

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // this handles all other cases besides the above
}
```

> The _ pattern will match any value. By putting it after our other arms, the _ will match all the possible cases that aren’t specified before it. The () is just the unit value, so nothing will happen in the _ case. As a result, we can say that we want to do nothing for all the possible values that we don’t list before the _ placeholder.

> However, the match expression can be a bit wordy in a situation in which we only care about one of the cases. For this situation, Rust provides `if let`.
