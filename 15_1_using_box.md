# Using `Box<T>` to Point to Data on the Heap

The simplest smart pointer is a box, `Box<T>`

- it allows you to store data on the heap, instead of the stack. The stack will maintain a pointer to the actual data on the heap.  

`Box<T>` is used when:

1. you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size. 

- This is explained in `"Enabling Recursive Types with Boxes"`.

2. you have a large amount of data and you want to transfer ownership, but ensure the data won’t be copied when you do so

¡hahá! ^^

- (this is the guts of the operation) Transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap. 

3. you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type  

- The third case is known as a trait object, discussed in Chapter 17

## Using a `Box<T>` to Store Data on the Heap

```rust
fn main() {
    // the value of `b` is stored on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}
```

We define the variable `b` to have the value of a Box that points to the value `5`, which is allocated on the heap. This program will print `b = 5`; in this case, we can access the data in the box similar to how we would if this data were on the stack. Just like any owned value, when a box goes out of scope, as `b` does at the end of main, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).

#### Let’s look at a case where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.

### Enabling Recursive Types with Boxes

At compile time, a recursive type's size cannot be known although Rust needs to know how much space a type takes up. This is because a recursive type can have a value that has as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

Let’s explore the `cons` list, which is a data type common in functional programming languages, as an example of a recursive type. The `cons` list type we’ll define is straightforward except for the recursion; therefore, the concepts in the example we’ll work with will be useful any time you get into more complex situations involving recursive types.  

### More Information About the `Cons` List

- A `cons` list is a data structure from the Lisp programming language and its dialects. 
- In Lisp, the cons function (short for "construct function") constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.  
- "to cons x onto y" informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y.


- Each item in a `cons` list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item. 
- A `cons` list is produced by recursively calling the `cons` function. The canonical name to denote the base case of the recursion is Nil, which is different from the concept of `null` discussed in Chapter 6.
- The `cons` list isn’t a commonly used data structure in Rust. Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use. 
- Other, more complex recursive data types are useful in various situations, but by starting with the `cons` list, we can explore how boxes let us define a recursive data type without much distraction.

```rust
// This code does not compile!
enum List {
    Cons(i32, List),
    Nil,
}
```

Note: We’re implementing a cons list that holds only i32 values for the purposes of this example. We could have implemented it using generics, as we discussed in Chapter 10, to define a cons list type that could store values of any type.

Using the List type to store the list 1, 2, 3 would look like the code in Listing 15-3:

```rust
// This code does not compile!
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

- The first `Cons` value holds 1 and another List value. This List value is another `Cons` value that holds 2 and another List value. 
- This List value is one more `Cons` value that holds 3 and a List value, which is finally Nil, the non-recursive variant that signals the end of the list.

#### The error we get when attempting to define a recursive enum

```bash
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ----- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

The error shows this type "has infinite size." The reason is that we’ve defined List with a variant that is recursive: it holds another value of itself directly. As a result, Rust can’t figure out how much space it needs to store a List value. Let’s break down why we get this error a bit. First, let’s look at how Rust decides how much space it needs to store a value of a non-recursive type.  

### Computing the Size of a Non-Recursive Type

Remember this enum from chapter 6?

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a `Message` value, Rust does this:

- goes through each of the variants to see which variant needs the most space
  - `Message::Quit` doesn’t need any space, 
  - `Message::Move` needs enough space to store two i32 values, 
  - the most space a `Message` value needs is the space to store the largest variant.

Contrast this with how much space Rust decides a recursive type like the `List` enum.

- The compiler starts by looking at the `Cons` variant, which holds a value of type `i32` and a value of type `List`. Therefore, `Cons` needs an amount of space equal to the size of an `i32` plus the size of a `List`. To figure out how much memory the `List` type needs, the compiler looks at the variants, starting with the `Cons` variant. The `Cons` variant holds a value of type `i32` and a value of type `List`, and this process continues infinitely

### Using `Box<T>` to Get a Recursive Type with a Known Size

The Rust compiler needs to know how much space to allocate for recursively defined types, so the compiler gives the error in Listing 15-4, including this helpful suggestion:

```bash
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

> In this suggestion, "indirection" means that instead of storing a value directly, we’ll change the data structure to store the value indirectly by storing a pointer to the value instead.

`Box<T>` is a pointer, so:

- Rust always knows how much space a `Box<T>` needs
- a pointer’s size doesn’t change based on the amount of data it’s pointing to. 
  
This means we can put a `Box<T>` inside the `Cons` variant instead of another `List` value directly. The `Box<T>` will point to the next List value that will be on the heap rather than inside the `Cons` variant. 

- Conceptually this is still a list, created with lists "holding" other lists, but now more like placing the items next to one another rather than inside one another.

```rust
// this will compile 😃
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

The `Cons` variant will need the size of an `i32` plus the space to store the box’s pointer data. The `Nil` variant stores no values, so it needs less space than the `Cons` variant. We now know that any List value will take up the size of an `i32` plus the size of a box’s pointer data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value __on the heap__, explicitly.

### A finite `Cons` list

Boxes provide only the indirection and heap allocation

- they don’t have any other special capabilities, like the other smart pointer types. 
- They also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need. We’ll look at more use cases for boxes in Chapter 17, too.

The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation. Let’s explore these two traits in more detail. These two traits will be even more important to the functionality provided by the other smart pointer types we’ll discuss in the rest of this chapter.
