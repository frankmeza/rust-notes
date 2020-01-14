# Processing a Series of Items with Iterators

- the iterator pattern is used over a collection in rust, like a mapping, filtering, or finding object, that still needs to be called by other code, aka _lazy_. Or it's like a manager of other code.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got {} as value", val);
}
```

This allows Rust developers to not have to use a manual iterator, aka for loop, like in JS:

```javascript
const sum = 0
const array = [1, 2, 3]

for (i = 0; i <= array.length; i++) {
    console.log(`Got ${array[0] value`)
} 
```

## The `Iterator` Trait and the `next` Method

- all iterators implement the `Iterator` trait:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

- new syntax here: `type Item` and `Self::Item`, which represents an _associated type_.
- the code reads like "To use this trait, you have to do your part too and define these methods down here for yourself."
  - explicitly, that a type `Item` is defined, and that this type is to be returned from the implementation of `next` wrapped in `Option<T>`.

### More About Iterators

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // this has to be mutable, in order to 
    // maintain state of where the iterator is
    let mut v1_iter = v1.iter();

    // these are each immutable refs to the values in the vector, per `iter()`
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```
- If we want to create an iterator that takes ownership of v1 and returns owned values, we can call `into_iter` instead of `iter`. Similarly, if we want to iterate over mutable references, we can call `iter_mut` instead of `iter`.

- behind the scenes, this code makes `v1_iter` mutable.

```rust
// the loop takes ownership
for val in v1_iter {
    println!("Got {} as value");
}
```

## Methods that Consume the Iterator

Methods that call `next` are called *consuming adaptors* because they consume the iterator.

```rust
#[test]
fn iterator_sum() {
    // create vector
    let v1 = vec![1, 2, 3];
    // create iterator
    let v1_iter = v1.iter();

    // `sum()` takes ownership, so `total` here owns the data
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

## Methods that Produce Other Iterators

- it's possible to chain multiple iterators together in a readable way

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1); // this will actually do nothing yet
```

- the method `collect()` needs to be called on iterator methods in order to actually fire the computation

## Using Closures that Capture Their Environment

```rust
#[derive(Partial, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter()
      .filter(|s| s.size == shoe_size)
      .collect();
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker")},
        Shoe { size: 13, style: String::from("sandal")},
        Shoe { size: 10, style: String::from("boot")},
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    let expected_shoes = vec![
        Shoe { size: 10, style: String::from("sneaker")},
        Shoe { size: 10, style: String::from("boot")},
    ];

    assert_eq!(in_my_size, expected_shoes);
}

```

- `shoes_in_my_size` fn takes ownership of `shoes` vector and a numeric size, returns filter results as `Vec<Shoe>`.
- `into_iter()` creates an iterator that takes ownership of the vector, then filter the results

## Creating Our Own Iterators with the Iterator Trait

- iterators are created from using `iter`, `into_iter`, or `iter_mut` on a vector
- these functions can also be called on other collection types, like hash maps
- custom iterators can be implemented with only a definition for `next`

An example use of this would be the iterative part of an ingress function that receives a vector of raw DTOs as JSON.  

```rust
// create iterator to count from 1 - 5
// 1. create a struct to hold values
// 2. create a constructor fn
// 3. then implement `Iterator` by defining a method for `next()`

// #1
struct Counter {
    count: u32,
}

// #2
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// #3
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
      self.count += 1;

      if self.count < 6 {
          Some(self.count)
      } else {
          None
      }
  }
}
```

## Using Our Counter Iterator’s `next` Method

```rust
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    // `.next()` called each time
    assert_eq!(counter.next(), Some(1)); 
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
}

```

## Using Other Iterator Trait Methods

- we can now use the `Iterator` trait methods, simply because we implemented `next`, a là:

```rust
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}