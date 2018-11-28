# Method Syntax

- methods are similar to funtions in syntax
- but they are defined within the context of a struct, inside of `impl {}`
- the first parameter is always `self`

## Defining Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // this is how we can define methods on a struct
    fn area(&self) -> u32 { // this is a method defined on Rectangle
        self.width * self.height
    }
}

fn main() {
    // an instance of a struct is defined
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the Rectangle is {} square pixels.",
        rect1.area() // this is where the method `area` is called
    );
}

```

- methods can still take ownership of `self`, borrow immutable instance with `&self`, or borrow a mutable copy `&mut self`
- `&self` is used here to have `area` not take ownership

### Some Stuff About C

#### Where’s the `->` Operator?

> In C and C++, two different operators are used for calling methods: you use `.` if you’re calling a method on the object directly and `->` if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().

> Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

> Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

> The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading `&self`, mutating `&mut self`, or consuming `self`. The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

### Methods with More Parameters

### Associated Functions

### Multiple `impl` Blocks

> But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.
