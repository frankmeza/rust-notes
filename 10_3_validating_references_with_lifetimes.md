# Validating References with Lifetimes

- in Rust, every reference has a lifetime, which is its scope of validity
- most lifetimes are implied and inferred, like types

The point of lifetimes is to ensure that the lifetime of a variable will be valid at runtime.

## Preventing Dangling References with Lifetimes

- a lifetime prevents dangling references, its main point

```rust
{                           // outer scope begins
    let r;                  // r is created, and is valid

    {                       // inner scope begins
        let x = 5;          // x is created, and is set to 5
        r = &x;             // r is set as a reference to value of x
    }                       // inner scope ends, x is now deallocated and out of scope

    println!("r: {}", r);   // error: r can no longer serve as a ref to x
}                           // outer scope ends
```

## The Borrow Checker

- Rust checks the lifetimes of refs to other values

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

- above, `x` has lifetime `'b`, which is longer than its ref `r` with lifetime `'a`

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);       // string1 is longer
}
```

- `string1` is a slice of a `String`
- `string2` is a string literal

Let's implement `longest()`:

```rust
// this won't compile...
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- the compiler expects a "lifetime parameter":

```
= help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
```

- neither we nor the compiler know which of x and y the return type refers to

## Lifetime Annotation Syntax

- lifetime annotations "describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

> Here are some examples: a reference to an `i32` without a lifetime parameter, a reference to an `i32` that has a lifetime parameter named `'a`, and a mutable reference to an `i32` that also has the lifetime `'a`.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in Function Signatures

```rust
fn longest(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- for lifetime `'a`, `fn longest` takes two params, both string slices that have lifetimes equal to `'a`
- this is us telling the compiler what to do, not the other way around
- these annotations only go in the fn signature, not the body

> When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

= = = = =

> When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words, the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

> Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes. Listing 10-23 is a straightforward example.
