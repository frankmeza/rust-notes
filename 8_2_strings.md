# Storing UTF-8 Encoded Text with Strings

- this chapter will cover CRUD operations on Strings
- also: how Strings are different from other collection types

## What is a String?

Rust has one string type: the string slice `str`, which is usually seen in its borrowed form `&str`.  

- `"string literals"` are string slices
- referring to strings in Rust usually refers to both `String` and `str` string literals, both are UTF-8 encoded

## Creating a New String

```rust
let mut s = String::new();
```

- this creates a new mutable, empty string `s`

### Creating a string with Content

```rust
let data = "initial contents";

// #to_string is available on types with Display trait

// these are equal, and all create String
let s = data.to_string();
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

## Updating a String

- similar to `Vec<T>`, you can push more data into a `String`

### Appending to a String with `push_str` and `push`

#### `push_str`

```rust
let mut s = String::from("foo");
s.push_str("bar"); 
// s == "foobar" now
```

- `#push_str` takes a string slice so that we don't take ownership

#### `push`

```rust
let mut s = String::from("lo");
s.push('l'); // #push only accepts a single character!
```

## Concatenation with the + Operator or the format! Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
// note that s1 has been moved here and can no longer be used
let s3 = s1 + &s2; // `s1` has been moved here but `&s2` is a reference
```

- imagine that `s1` upgrades itself and becomes `s3` by adding the reference to `s2`, i.e. it returns ownership of the result
- so: after defining `s3`, `s1` is no longer valid because it has been moved
- NB: `s2` is `&String`... the compiler can coerce `&String` into `&str`
- so: `s2` is still valid after all of this...!

### the `format!` macro

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

- `format!` does not take ownership of anything
- SO: all variables are still valid

https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings
