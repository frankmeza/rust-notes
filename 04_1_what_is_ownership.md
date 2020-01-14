# What is Ownership in Rust?

- Ownership is a central feature of the language.
- In Rust, "memory is managed through a system of ownership with a set of rules that the compiler checks at compile time."

## The Stack and the Heap

- these are the parts of memory.

### The Stack

- stores values in the order it gets them, aka *Last In, First Out*, aka **LIFO**.
- think of a stack of plates - you add to the top and take from the top, never from the middle.
- adding and removing data from here are called respectively, **pushing** onto the stack, and **popping** off the stack.
- accessing data from the stack is fast because the stack only ever puts data on the top.
- all data on the stack must take up a known, fixed size.

### The Heap

- can accommodate data of unknown size at compile time.
- less organized than the stack, and/or is more complex,
- the OS finds a large enough spot in memory, marks it as being used, and returns a **pointer**. This is called **allocating on the heap**, aka allocating.
- because the pointer is a known, fixed-size amount of data, *it* can itself be stored on the stack.
- when the pointer is used from the stack, it must be eventually used to get the data from the heap.
- accessing data from the heap is slow because you have to follow a pointer to get to the actual data.

### The Underlying Ideas behind Ownership

- Do not seem extremely difficult, it's just a novel concept for a JS developer working in a browser or mobile view.

## Ownership Rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### First example of ownership

```rust
fn some_function() {
    let s = "hello"; // s is valid from this point forward
    // ... do stuff with s...
}   // this scope is now over, and s is no longer valid
```

1. When `s` comes into scope, it is valid.

It remains valid until it goes out of scope.

### The `String` Type

```rust
let s = String::from("hello");
```

This kind of string can be mutated:

```rust
let mut s = String::from("hello"); // "hello" is like a starter string

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

...but string literals cannot be mutatated. The contents of them are known at compile time, as a scalar type that is stored on the stack.

```rust
// so this is not mutable
let string_literal = "you can't change me";
```

### Rust stores `String` objects on the heap.

This is to support a mutable, growable piece of text, which cannot be stored on the stack.

Rust has no garbage collector.

**The memory of data is automatically returned once its owner goes out of scope.**

---

## Ways Variables and Data Interact: Move

### What's Really Happening Here?

It looks like we are just binding `s2` to the *value* of `s1`, but this is not the case.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

In reality, a `String` in Rust has four parts, in key and value style:

- `name` - the variable name
- `len` - how much memory in bytes that the `String` is using
- `cap` (capacity) - another property that will be discussed later
- `ptr` (pointer) - the address of the entire value, on the heap, as a char array (values with indices)

#### This data is on the stack, for the `String`

| name  | value |
|-------|-------|
| name  | 5     |
| ptr   | -> points to next table|
| len   | 5     |
| cap   | 5     |

NB: notice the `len` and `cap` properties? That seems suspiciously like an array...

#### The actual data for the `String` is on the heap

| index | value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

So when `s1` is assigned to `s2`, now there will be two objects on the stack (like the first table) that both point to the same data in the stack (the second table). They do not create identical datasets on the heap.

> When a variable goes out of scope, Rust calls `drop()`to clean up the heap memory for that variable. But with two variables pointing to the same place in the heap, this is a problem: when `s2` and `s1` go out of scope, they both try to free the same memory. This is a `double free error` and is one of the memory safety bugs we mentioned previously, and can lead to memory corruption, and security vulnerabilities.

> To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust considers `s1` to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use `s1` after `s2` is created; it won’t work:

```rust
let s1 = String::from("hello");
let s2 = s1; // it's game over for `s1`, it's no longer valid

println!("{}, world!", s1);
```

You’ll get an error like this because Rust prevents you from using the invalidated reference:

```bash
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

> If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. Here we would read this by saying that s1 was moved into s2. So what actually happens is shown in Figure 4-4 (above).

### If You *Do* Want to Deeply Copy a Heap Value

... you can use `clone`, like this.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();    // here is where the difference is, similar in my mind to slice() in JS

println!("s1 = {}, s2 = {}", s1, s2);
```

Much of this does NOT apply to integers because their byte size is known at compile time. There's no reason to use `clone()` with integers, because numbers just get shallow copies anyway. Rust has a `Copy` trait on integers on the stack.

Any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`. Here are some of the types that are `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, bool, with values `true` and `false`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, but only if they contain types that are also `Copy`. For example, `(i32, i32)` is `Copy`, but `(i32, String)` is not.

```rust
fn main() {  // comparing the two types: "Copy" and non-"Copy" types
    // non-copy
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // copy
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we use `s` after the call to takes_ownership, Rust would throw a compile time error. These static checks protect us from mistakes. Try adding code to main that uses `s` and `x` to see where you can use them and where the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    // initial assignment
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

// to me, this feels like it should be called
// `fn conjures_from_the_ether_and_bestows_ownership_upon_a_variable
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.

// this could be called transfers_ownership_from_x_to_new_x
// this is like a middleware shell for the String type
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}
```

> Ownership of a variable follows the same pattern every time: **assigning a value to another variable moves it**. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop()` unless the data has been moved to be owned by another variable.

> Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

It’s possible to return multiple values using a tuple, like this:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

But this is much boilerplate for a concept that should be common. Luckily for us, Rust has a feature for this concept, called references.
