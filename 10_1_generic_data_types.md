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

