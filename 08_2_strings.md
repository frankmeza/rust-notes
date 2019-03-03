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

## Indexing into Strings

### WRONG

```rust
let s1 = String::from("hello");
let h = s1[0];
```

### RIGHT

- `String` is a wrapper over a `Vec<8>`

```rust
let len = String::from("Hola").len() // Hola is 4 bytes long.

let len = String::from("Здравствуйте").len(); // Здравствуйте is 24


```

> Asked how long the string is, you might say 12. However, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value. To demonstrate, consider this invalid Rust code:

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

> What should the value of answer be? Should it be З, the first letter? When encoded in UTF-8, the first byte of З is 208 and the second is 151, so answer should in fact be 208, but 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even if the string contains only Latin letters: if &"hello"[0] were valid code that returned the byte value, it would return 104, not h. To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

## Bytes and Scalar Values and Grapheme Clusters! Oh My!

- utf-8 related, there are three relevant ways to look at strings: bytes, scalar values, grapheme clusters
- in Devanagari script `“नमस्ते”`, is stored as `Vec<u8>`: `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]` and is 18 bytes long

TLDR; Rust just doesn't let you index strings by position

## Slicing Strings

- between web stuff and systems stuff, it's impossible to know which data type to expect:
    - byte
    - character
    - grapheme cluster (accent marks)
    - string slice

```rust
let hello = "Здравствуйте";

let s = &hello[0..4]; // s will be &str containing first 4 bytes of `hello`
```

- using ranges like the above to create string slices is not a good idea

### Methods for Iterating Over Strings

```rust
// iterates over characters
for c in "नमस्ते".chars() {
    println!("{}", c);
}

// iterates over bytes
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```
