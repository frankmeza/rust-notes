# What is Ownership in Rust?

- this is a central feature of the language
- in Rust, `memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.`

## The Stack and the Heap

- these are parts of memory

### The Stack

- stores values in the order it gets them, aka *Last In, First Out*, aka **LIFO**.
- think of a stack of plates - you add to the top and take from the top, never from the middle.
- adding and removing data from here are called respectively, **pushing** onto the stack, and **popping** off the stack.
- accessing data from the stack is fast because the stack only ever puts data on the top.
- all data on the stack must take up a known, fixed size.

### The Heap

- can accommodate data of unknown size at compile time.
- less organized than the stack, or more complicated,
- the OS finds a large enough spot in memory, marks it as being used, and returns a **pointer**. This is called **allocating on the heap**, aka allocating.
- because the pointer is a known, fixed-size amount of data, *it* can itself be stored on the stack.
- when the pointer is used from the stack, it must be eventually used to get the data from the heap.
- accessing data from the heap is slow because you have to follow a pointer to get to the actual data

#### The Underlying Ideas behind Ownership

- do not seem extremely difficult, it's just a novel concept for a JS developer working in a browser or mobile view

## Ownership Rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

I left off at https://doc.rust-lang.org/stable/book/2018-edition/ch04-01-what-is-ownership.html#ownership-rules
