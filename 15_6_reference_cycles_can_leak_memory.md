# Reference Cycles Can Leak Memory

### Rust’s memory safety

- its guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak).
- preventing memory leaks entirely is not one of Rust’s guarantees in the same way that disallowing data races at compile time is, meaning memory leaks are memory safe in Rust.
- Rust does allow memory leaks by using `Rc<T>` and `RefCell<T>`: it’s possible to create references where items refer to each other in a cycle.
- This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

## Creating a Reference Cycle (This is bad, don't do it)

```rust
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

// A `Cons` list definition that holds a RefCell<T>
// so we can modify what a Cons variant is referring to
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

another variant of the `List`! 

- The second element in the `Cons` variant is now `RefCell<Rc<List>>`,
- so instead of having the ability to modify the `i32` value earlier,
- we want to modify which `List` value a `Cons` variant is pointing to.
- We’re also adding a `tail` method to make it convenient for us to access the second item if we have a `Cons` variant.

```rust
// Creating a reference cycle of two List values pointing to each other
fn main() {
    // 1. `Rc<List>` with a `List` value in `a` with an initial list of `5`, `Nil`
    let a = Rc::new(
        Cons(5, RefCell::new( // this line is the `List`
            Rc::new(Nil),
        )),
    );

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // 2. `Rc<List>` holding another `List` value in `b` with value `10` and points to list `a`.
    let b = Rc::new(
        Cons(10, RefCell::new( // this line is the `List`
            Rc::clone(&a),
        )),
    );

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
      *link.borrow_mut() = Rc::clone(&b); // this is where ref cycle begins
    }

    // where a increments b, and where b increments a in a cycle
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

- create a list in `a`, and a list in `b` that points to the list in `a`
- then it modifies the list in `a` to point to `b`, creating a reference cycle
- there are `println!` statements along to document things
   
1. We create an instance of `Rc<List>` with a `List` value in `a` with an initial list of `5`, `Nil`.
2. then create an `Rc<List>` instance holding another `List` value in `b` that contains the value `10` and points to the list in `a`.

We modify `a` so it points to `b` instead of `Nil`, creating a cycle. We do that by using the `tail` method to get a ref to the `RefCell<Rc<List>>` in a, which we put in the variable link. Then we use the borrow_mut method on the `RefCell<Rc<List>>` to change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in `b`.

we’ll get this output:

```
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2

b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

b rc count after changing a = 2
a rc count after changing a = 2
```

- the ref count of the `Rc<List>` instances in both `a` and `b` are `2` after we change the list in `a` to point to `b`
- at end of scope, Rust will try to drop `b` first, which will decrease the count of the `Rc<List>` instance in `b` by `1`.
- but, because `a` is still referencing the `Rc<List>` that was in `b`, that `Rc<List>` has a count of `1` rather than `0`, 
  - so the memory the `Rc<List>` has on the heap won’t be dropped. The memory will just sit there with a count of `1`, forever. 

```
|------|         |------|
|  a   | ----->  |   5  |
|      |         |      |  
|------|         |------|  

  ^                   |
  |                   |
  |                   |
  |                   |  
  |                   v

|------|         |------|
|      |         |      |
| 10   | <----   |  b   |  
|------|         |------|
```
