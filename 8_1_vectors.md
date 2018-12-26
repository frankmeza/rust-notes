# Storing Lists of Values with Vectors

- these seem very similar to TS arrays, whose values must be typed and of the same type

```rust
let v: Vec<i32> = Vec::new();
```

The above must be typed (the `<i32>`) because it has no values yet and so Rust doesn't know what kind of element to expect within this vector in the future.  

This is not usually necessary because Rust can infer the data type based on the values at creation time.

```rust
let v = vec![1, 2, 3];
```

## Updating a Vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
```

- first, a mutable vector is created
- then the vector has values pushed into it. This is where Rust is able to infer the data types based on the pushed values.
- so in this example, the strong typing is not necessary

## Dropping a Vector Drops Its Elements

```rust

{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
}   // <- v goes out of scope and is freed here
```

## Reading Elements of Vectors

There are two ways to read elements from a vector:  

- using bracket notation with the wanted index,
- using `get` with the same index.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // bracket notation used here
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("Third element is {}", third),
    None => println!("There is no third element"),
}
```

### Bracket Notation `&[index]` and Using `get`

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100]; // panics
let does_not_exist = v.get(100); // does not panic
```

#### Bracket Notation

- When using bracket notation, the `&` must be used with it to give us a reference to that item. Will panic if index does not exist

#### `get`

- `get` can be used instead, which gives us `Option<&T>` . Won't panic, will return `None`.
- Use this method if accessing an element beyond the range of the vector happens normally.
- then have logic to handle having either `Some(&element)` or `None`

> For example, the index could be coming from a person entering a number. If they accidentally enter a number that’s too large and the program gets a None value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value. That would be more user-friendly than crashing the program due to a typo!

#### References: Mutable and Immutable

```rust
let mut v = vec![1, 2, 3, 4, 5];
// this is an immutable reference to the first element
let first = &v[0];

v.push(6);
println!("First element is {}", first);
```

- these cannot exist in the same scope
- the code will not compile

Here's why:

> ...why should a reference to the first element care about what changes at the end of the vector? This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

 ## Iterating over the Values in a Vector

### Immutable

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i); // prints immutable references
}
```

### Mutable

```rust
let mut v = vec![100, 32, 57];
// "for item `i` in immutable reference to mutable vector"
for i in &mut v {
    *i += 50; // adds 50 to each item
}
```

- RE: the `*i` -> "To change the value that the mutable reference refers to (using the `+=`), we have to use the dereference operator `(*)` to get to the value in i before we can use the `+=` operator. We’ll talk more about `*` in Chapter 15."

## Using an Enum to Store Multiple Types

- normally vectors can only store values of the same type
- enums are sort of an exception because the variants are all defined under the same enum type

```rust
enum SpreadsheetCell {
    // the variants are of different data types
    Int(i32),
    Float(f64),
    Text(String),
}

// ...but Rust allows us to store them all together, each as a SpreadsheetCell variant
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

This is permissible because Rust will be able to know how much memory on the heap to allocate and also to know if a certain operation will cause an error on a given variant's data type.

> Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled, as discussed in Chapter 6.

- an alternative to this approach would be to use traits, which will be discussed later in chapter 17.