# References and Borrowing

```rust
fn main() {
    let s1 = String::from("hello");

    // here, calculate_length is only using &s1
    // as a reference to assign a value to `len`
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // a reference to a pointer to data on the heap
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

> The scope in which the variable `s` is valid is the same as any function parameter’s scope, but we don’t drop what the reference points to when it goes out of scope because we don’t have ownership. When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

- `&String` is a reference to a pointer (is the pointer on the stack?) to data on the heap.

> We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

- in the above, a *reference* to `s1` is passed into `calculate_length`, instead of the pointer itself.
- the ampersand begins with 'a', so does address (pneumonic device)
- or you can think of the `&` as a mask that one variable wears of another variable

## Referencing

- Aside: the opposite of referencing is dereferencing, with `*` (discussed in Chapter 8)

```rust
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

```

- `len` does not OWN the value of `s1`
- so when `len` goes out of scope, nothing happens to the owner of the `&s1` value passed into it.
- `calculate_length` only *borrows* the value of `s1` by using `&s1` as a reference

## Mutable References

### Right ✅

```rust
fn main() {
    let mut s = String::from("hello"); // assigned as mutable with `mut`

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### WRONG 🚫

```rust
fn main() {
    // this is not marked `mut`
    let s = String::from("hello");

    change(&s); // boom boom uh oh
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

> But mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope. This code will fail:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```bash
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
  ```

- what if `r1` and `r2` independently mutate the value of `s`? whose change will be final?

### Data Race

A *data race* is similar to race conditions, and happens when these three conditions occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

> Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!

> As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    println!("{}", r1); // hello
    r1.push_str(" wut");

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

println!("{}", r2); // hello wut
```

> A similar rule exists for combining mutable and immutable references. This code results in an error:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem, it's just a silly double reference.
// It's like hearing two hypothetical stories with the same initial value, no problem

let r3 = &mut s; // BIG PROBLEM
```

Here’s the error:

```bash
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

- having both an immutable *and* mutable reference in the same scope is not allowed.
- **but having multiple immutable references is okay**

### Dangling References

> In languages with pointers, it’s easy to erroneously create a dangling pointer, a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: **if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.**

Let’s try to create a dangling reference, which Rust will prevent with a compile-time error:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

> This error message refers to a feature we haven’t covered yet: lifetimes. We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem:


> This function's return type contains a borrowed value, but there is no value for it to be borrowed from. Let’s take a closer look at exactly what’s happening at each stage of our dangle code:

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

>Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

The solution here is to return the String directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

- At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
- References must always be valid. ie, they cannot point to expired data

Next, we’ll look at a different kind of reference: slices.
