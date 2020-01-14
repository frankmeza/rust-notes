# Defining and Instantiating Structs

- similar to tuples in aggregating data, but with unique keys to single values
- the idea is identical to javascript/typescript
- keyboard is `Struct`

## Struct Definition

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

### Instances of a Struct, Immutable and Mutable

```rust
// an instance of User struct

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

// a mutable instance of User //

// notice use of `mut` keyword
let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// can be used to set a value in a mutable instance
user2.email = String::from("anotheremail@example.com");
```

- dot notation can be used to get a value of a key

### Example of a Function Returning an Instance of a Struct

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

- notice the absence of the `;` in order to return the struct from the function

### ES6 Similarities

#### Same Name Params and Keys

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

- if a param name and a struct field have the same name, you can use the shorthand shown above

#### Spread Operator Equivalent, `Struct Update` Syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

- A small difference is that the order of the "spread" versus the dynamic traits does not matter as in JS/TS, as far as spreading first and then overwriting with new stuff

## Tuple Structs without Named Fields to Create Different Types

- tuple structs
- have the added meaning that `struct` provides, but don’t have names associated with their fields only the types of the fields
- useful when you giving the whole tuple a name and make the tuple be a different type than other tuples, when defining a `struct` would be overkill

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

- `black` and `origin` values are different types, being instances of different tuple structs
- each `struct` you define is its own type, even though the fields within the struct have the same types.

> For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces, you can use a . followed by the index to access an individual value, and so on.

## Unit-Like Structs Without Any Fields

- these are used when you need to implement a `trait` on a type, but do not want to store any data in the type itself (discussed later in Chapter 10)

```rust
// my best guess without reading ahead
struct nil_struct();
```

## Ownership of Struct Data

- in `User`, `String` type rather than the `&str` string slice type is used
- this is deliberate because we want instances of this struct to own all its own data and for that data to be valid for as long as the entire struct is valid.

> It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:

```rust
struct User {
    username: &str, // error: expected lifetime parameter
    email: &str,    // then here too
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```