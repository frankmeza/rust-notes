# Treating Smart Pointers Like Regular References with the Deref Trait

By implementing `Deref`, you can:

- customize the behavior of the dereference operator, `*` (as opposed to the multiplication or glob operator). 
- implement `Deref` so a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

wut??

- first see how the dereference operator works with regular references
- then we define a custom type that behaves like Box<T>, and see why the deref operator doesn’t work like a reference on our newly defined type
- then see how implementing the `Deref` trait makes it possible for smart pointers to work in ways similar to references. 
- Then we’ll look at Rust’s deref coercion feature and how it lets us work with either references or smart pointers.

Note: there’s one big difference between the `MyBox<T>` type we’re about to build and the real `Box<T>`: our version will not store its data on the heap. We are focusing this example on `Deref`, so where the data is actually stored is less important than the pointer-like behavior.

## Following the Pointer to the Value with the Dereference Operator

A regular reference is a type of pointer, think of a pointer is as an arrow to a value stored somewhere else. 

```rust
fn main() {
    // make a ref to an i32 value,
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // and then use the dereference operator (*) to follow the reference to the data
    assert_eq!(5, *y);
}
```

- `x` holds i32 value, `5`
- We set `y` equal to a reference to `x`. We can assert that `x` is equal to `5`. 
- to make an assertion about the value in `y`, we have to use `*y` to follow the reference to the value it’s pointing to (hence dereference). Once we dereference `y`, we have access to the integer value y is pointing to that we can compare with `5`.

`assert_eq!(5, y);` instead returns this compilation error:

```
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`
```

Comparing a number and a reference to a number isn’t allowed because they’re different types. We must use the dereference operator to follow the reference to the value it’s pointing to.

## Using Box<T> Like a Reference

We can rewrite the code above to use a Box<T> instead of a reference;
```rust
// dereferencing a Box<T>
fn main() {
    let x = 5;
    let y = Box::new(x); // instead of let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- The only difference is that we use a box pointing to the value in `x` rather than a reference pointing to the value of `x`
- we can use the dereference operator to follow the box’s pointer in the same way that we did when `y` was a reference. Next, we’ll explore what is special about `Box<T>` that enables us to use the dereference operator by defining our own box type.

## Defining Our Own Smart Pointer

## Treating a Type Like a Reference by Implementing the Deref Trait

## Implicit Deref Coercions with Functions and Methods

## How Deref Coercion Interacts with Mutability
