# An Example Program Using Structs

## Different Approaches: Variables, Tuples, Structs

### Using (Immutable) Variables

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- the variables `width1` and `height1` are just blowin' in the wind
- `fn area` is calculated with each being passed in separately
- the function signature `fn area(width: u32, height: u32) -> u32` is meant to receive one rectangle, but receives two parameters
- it's better to pass height and width in together as part of a single object

### The Same Thing Using Tuples

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

- now only one param is being passed into `fn area()`
- but we don't know which is height and which is width

### Finally, Using Structs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

- the `struct Rectangle` has descriptive fields `height: u32` and `width: u32`
- `rect1` is instantiated as a `Rectangle` with values 30, and 50
- `fn area` now takes one parameter: `&Rectangle`, "an immutable borrow of a struct Rectangle instance."

 > As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using `rect1`, which is the reason we use the `&` in the function signature and where we call the function.

- `fn area` accesses width and height from the Rectangle instance passed to it
- the function signature for `fn area` calculates the area of Rectangle, with width and height
- we see that `width` and `height` are related, and gives descriptive names to the values instead of tuple index values of 0 and 1, much clearer

## Adding Useful Functionality with Derived Traits

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

- with this code, we will get an error on `println!("rect1 is {}", rect1);`
- essentially, Rust doesn't know enough about this object's type, and you will receive this error:

```bash
    `Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string`
```

Then, with this code in place:

```rust
// ... other code

fn main() {
    // ... other code

    println!("rect1 is {:?}", rect1);
}
```

... now you get `rect1 is Rectangle { width: 30, height: 50 }`

*COOL!*

### More Formatting

> When we use the {:#?} style in the example, the output will look like this:

```rust
rect1 is Rectangle {
    width: 30,
    height: 50
}
```

### Yet ANOTHER Way to Accomplish this Separation And Clarity in Rust Code...

> Our area function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.
