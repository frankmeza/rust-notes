# String Slices

- takes &String as a parameter.


- a reference to part of a String, like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // a ref to just the [0..5] part
let world = &s[6..11];  // a ref to the remainder
```

## Ranges

```rust
let range = [1..10];
```

- begins at second element in range, why?
- goes up to but does not include the `end` number?
- an inclusion of the final element uses this syntax: `let range = [..=10];`, so `[0..5] == [0..=4]`
- the syntax of this identical to that of Ruby or Python ranges
- you can drop both values to take a slice of the entire string, like this:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

## String Literals Are Slices

Instantiating a variable like this:

```rust
let s = "Hello, world!";
```

makes `s` a string literal, `&str`.

### Other Slices

> String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

> Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

> This slice has the type `&[i32]`. It works the same way as string slices do, by storing a reference to the first element and a length. You’ll use this kind of slice for all sorts of other collections. We’ll discuss these collections in detail when we talk about vectors in Chapter 8.
