# Validating References with Lifetimes

- in Rust, every reference has a lifetime, which is its scope of validity
- most lifetimes are implied and inferred, like types

The point of lifetimes is to ensure that the lifetime of a variable whose value is borrowed will be valid at runtime.

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

> When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words, the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

> Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes.

```rust
fn main() {
    // string1 in outer scope
    let string1 = String::from("long string is long");

    {
        // string2 in inner scope
        let string2 = String::from("xyz");
        // longest() here takes in &refs to variables in two different scopes
        // result is a ref to a value that only exists in inner scope,
        // which is okay because result only exists within the inner scope too
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

The above code works just fine. The following code, however, does not:

```rust
fn main() {
    // string1 is born
    let string1 = String::from("long string is long");
    //result is instantiated
    let result;

    {
        // string2 is born
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
//  ^ `string2` dropped here while still borrowed

    // result's reference to string2 is already gone
    println!("The longest string is {}", result);
}

```

- error ^^ : `error[E0597]: `string2` does not live long enough`

## Thinking in Terms of Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

The above code compiles fine, because only `x` has an explicit lifetime. The lifetime of `y` has no bearing on either the lifetime of `x` or the return value.

> When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function.

```rust
// This code does not compile!
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

The lifetime of the return type isn't related to parameters' lifetimes at all.

**Basically, the return value cannot refer to a value created within the function, as it will go out of scope at the end of the function block.**

## Lifetime Annotations in Struct Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel
        .split()
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };
}
```

- this struct has one field, `part` that holds a string slice, which is a reference.
- as with generic data types, the name of the lifetime parameter is defined inside of angle brackets `< >` to use the lifetime parameter in the body of the struct.
- this annotation means that an instance of `Important Excerpt` can't outlive the reference that it holds on to.

## Lifetime Elision

- every reference has a lifetime, and lifetime params need to be specified. However, this code compiles:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### Why?

Way back when, the signature would have had to be this:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

There exists *lifetime elision* rules. These are specific cases when the compiler knows already what you want to happen with lifetimes.

There are `input lifetimes` on function and method parameters, and `output lifetimes` on return values.

### The Three Rules of Lifetime Elision

These are for `fn` and `impl` blocks:

1. each param gets its own lifetime parameter, i.e. 2 params means two separate lifetime params.

2. if there is exactly one input lifetime param, that lifetime is assigned to all output lifetime params, i.e. `fn foo<'a>(x: &'a i32) -> &'a i32 {}`

3. if there are multiple input params, but one is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime params.

Let's pretend to be the compiler and examine a `fn` without explicit lifetime params:

### Example: A Function with One Parameter

```rust
fn first_word(s: &str) -> &str {}
```

- Rule 1: each param gets a lifetime. So now we have:

```rust
fn first_word<'a>(s: &'a str) -> &str {}
```

- Rule 2: This applies, because there's only one param, so:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {}
```

- Rule 3: Doesn't apply here.

### Example: A Function with Two Parameters

```rust
fn longest(x: &str, y: &str) -> &str {}
```

- Rule 1: each param gets a lifetime:

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```

- Rule 2: Doesn't apply here.
- Rule 3: Doesn't apply here.

## Lifetime Annotations in Method Definitions

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

- Lifetime names for struct fields are declared after the `impl`.
- `fn level` above only has one param, `&self`

### Example: Where the Third Rule of Elision Applies

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

- there are two input lifetimes, so each gets a lifetime param. Rule 1
- but one of the params is `&self`, so the return type gets the lifetime of `&self`

## The Static Lifetime

A special lifetime, `'static`, means that it is valid throughout the entire duration of the program. All string literals have `'static` lifetimes:

```rust
let s: &'static str = "I have a static lifetime";
```

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

> Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

> This is the longest function from Listing 10-22 that returns the longer of two string slices. But now it has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. This extra parameter will be printed before the function compares the lengths of the string slices, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

- `fn longest_with_an_announcement` returns the longest of two string slices.
- now it has another generic param, `ann: T` which can be any type that fulfills the `Display` trait.
- we are telling the compiler that `x` and `y` live equally as long

