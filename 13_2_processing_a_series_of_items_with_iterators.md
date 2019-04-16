# Processing a Series of Items with Iterators

- the iterator pattern is used over a collection in rust, like a mapping, filtering, or finding object, that still needs to be called by other code, aka _lazy_. Or it's like a manager of other code.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got {} as value");
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

