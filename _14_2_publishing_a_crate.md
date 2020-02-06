# Publishing a Crate to Crates.io

- `crate`s are Rust packages, distributable by `crates.io`

## Making Useful Documentation Comments

- Accurately documenting your packages is worth investing the time to write documentation. 
- Rust has a documentation comment, that will generate HTML documentation, for documentation comments for public API items intended for programmers interested in knowing how to use your crate as opposed to how your crate is implemented.
- Documentation comments use three slashes, `///`, instead of two and support Markdown notation for formatting the text. 
- Place documentation comments just before the item they’re documenting.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Things to touch on in comments include:

- Panics - when will your code panic?
- Errors - if a fn returns a `Result`, explain what errors can be present in this `Result`
- Safety - if `unsafe`, explain why