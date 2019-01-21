# Generic Data Types

## In Function Definitions


```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

- the generics are placed in the signature
- the function bodies are identical

- any identifier can be used as a generic type, however `T` is the de facto standard

The generic signature for a shared function would be:

`fn largest<T>(list: &[T]) -> T {}`

> We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a value of the same type T.

- the `<T>` signifies that the function is a generic function
- `list` is a slice of type `<T>`

## In Struct Definitions

```rust
// since only one type is <T> defined,
// both params must be of the same type
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 }; // i32
    let float = Point { x: 1.0, y: 4.0 }; // f32
}
```

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## In Enum Definitions

```rust
enum Option<T> {
    Some(T),
    None,
}
```

> This definition should now make more sense to you. As you can see, Option<T> is an enum that is generic over type T and has two variants: Some, which holds one value of type T, and a None variant that doesn’t hold any value. By using the Option<T> enum, we can express the abstract concept of having an optional value, and because Option<T> is generic, we can use this abstraction no matter what the type of the optional value is.

### Use with Multiple Types

```rust
enum Result<T, E> {
    Ok(T), // if Ok, return the type
    Err(E), // else, return the Err
}
```

## In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

// see note_A below
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 5 };

    println!("p.x = {}", p.x());
}
```

- above, we have a method `x` on `Point<T>` that returns a reference to the value in field `x`
- **note_A**: `<T>` must be appended to both `impl` and `Point` when writing completely generic methods on a `struct` that can accept a generic type

- the following can be implemented for just `Point<f32>`, like this:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

- this method only exists for instances of `Point<f32>` and not `Point<i32>`, for example.

## Even More Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

// implements over generic types T and U
// for Point with generic types T and U
impl<T, U> Point<T, U> {
    // mixup over generic types V and W
    // NB generic types T, U are known here
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 }; // create an instance of Point<i32, f64>
    let p2 = Point { x: "Hello", y: 'c' }; // create an instance of Point<str, char>

    let p3 = p1.mixup(p2); // create an instance of Point<i32, char>
}
```

## Optimized Through Rust Compiler "Monomorphization"

- generic code runs no slower than code with concrete types