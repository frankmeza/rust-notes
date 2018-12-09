# Concise Control Flow with `if let`

- `if let` acts just as a single `match` item

## Syntactical Sweets

### You can change this:

```rust
let some_u8_value = Some(0u8);

match some_u8_value {
    Some(3) => println!("three"),
    _ => (), // this lets ALL other cases go to nothing
}
```

### Into this:

```rust

if let Some(3) = some_u8_value {
    println!("three");
}
```

... which is just *so slick*.

it's summed up with:

```rust
if let <pattern> = <expression> {
    <code to execute>
}
```

## However, There is a Catch

> ...you can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values.

- hence, you lose the exhaustive type-checking that `match` offers

### Another Example

```rust
let mut count = 0;

match coin {
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state)
    },
    _ => count += 1,
}
```

### Versus The Following

```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

### Summary

- `Option<T>` allows the developer to work with handling null/`None` values