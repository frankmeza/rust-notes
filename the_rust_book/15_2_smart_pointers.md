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

> Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how smart pointers behave differently from references by default. Then we’ll look at how to add the ability to use the dereference operator.

The `Box<T>` type:
- is defined as a tuple struct with one element, and needs the new function defined on Box<T>.

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

We define a `struct` named `MyBox` and declare a generic parameter `T` so our type can hold values of any type

- the MyBox type is a tuple struct with one element of type T. 
- `The MyBox::new` function takes one parameter of type `T` and returns a `MyBox` instance that holds the value passed in.

-- `This is all kind of like middleware for a single variable...` --

Let’s use the `MyBox<T>` type we’ve defined instead of `Box<T>`

- The code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference `MyBox`


```rust
fn main() {
    // attempting to use MyBox<T> in the same way we used references and Box<T>: FAIL
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

WHY?? Here’s the resulting compilation error:

```
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^
```

Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that ability on our type.  

- to enable dereferencing with the * operator, we implement the `Deref` trait

## Treating a Type Like a Reference by Implementing the Deref Trait

- to implement a trait, provide implementations for the trait’s required methods
- The `Deref` trait from the standard library, requires us to implement one method named `deref` that 
  - borrows self,
  - and returns a reference to the inner data

```rust
// add import statement for trait to be linked to, for this struct
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // - borrows self,
    // - and returns a reference to the inner data
    fn deref(&self) -> &T {
        &self.0
    }
}
```

`type Target = T;` 

- defines an associated type for the `Deref` trait to use
- associated types are a different way of declaring a generic parameter, to be covered later
- fill in the body of the `deref` method with `&self.0` so `deref` returns a reference to the value we want to access with the `*` operator
- the `main` function above that calls `*` on the `MyBox<T>` value compiles now
- without the `Deref ` trait, the compiler can only dereference `&` references
- the `deref `method gives the compiler the ability to:
  - take a value of any type that implements `Deref `,
  - then call the `deref `method to get a `&` reference that it knows how to dereference.

Behind the scenes Rust actually ran this code:

```rust
*(y.deref())
```

Rust substitutes the `*` operator with a call to the `deref` method and then a plain dereference so we don’t have to think about whether or not we need to call the `deref` method. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements `Deref`.

`You can in many cases just write '*y'`

The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary, is the ownership system. 

wut??

If the `deref` method returned the value directly instead of a reference to the value, the value would be moved out of self. We don’t want to take ownership of the inner value inside `MyBox<T>` in this case or in most cases where we use the dereference operator.

Note that the `*` operator is replaced with a call to the `deref` method and then a call to the `*` operator just once, each time we use a `*` in our code.  

Because the substitution of the `*` operator does not recurse infinitely, we end up with data of type i32, as wanted.

### Implicit Deref Coercions with Functions and Methods

## How Deref Coercion Interacts with Mutability
