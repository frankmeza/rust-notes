# `RefCell<T>` and the Interior Mutability Pattern

## Interior mutability in Rust

- allows you to mutate data even when there are immutable references to that data
- normally, this action is disallowed by the borrowing rules

To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.  

We can use types that use the __interior mutability pattern__ when we:  
 
 - can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that
 - the unsafe code involved is then wrapped in a safe API, and the outer type is still immutable

## Enforcing Borrowing Rules at Runtime with `RefCell<T>`

- In contrast to `Rc<T>`, it represents single ownership over the data it holds
  
_So, what makes `RefCell<T>` different from a type like `Box<T>`?_  

Recall borrowing rules from Chapter 4:

- At any given time, you can have either (but not both of):
  - one mutable reference,
  - or any number of immutable references.
- References must always be valid.
  
With references and `Box<T>`, the borrowing rules’ invariants are enforced at compile time

- using `RefCell<T>`, these invariants are enforced at runtime
  
> With references, if you break these rules, you’ll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit.

---

 ### advantages of checking the borrowing rules at compile time

- errors will be caught sooner in the development process
- no impact on runtime performance because all the analysis is completed beforehand 

> For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.

---

### advantage of checking the borrowing rules at runtime instead

- certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks. 
- Static analysis, like the Rust compiler, is inherently conservative. Some properties of code are impossible to detect by analyzing the code: the most famous example is the "Halting Problem", which is beyond the scope of this book but is an interesting topic to research.

> Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program; in this way, it’s conservative. If Rust accepted an incorrect program, users wouldn’t be able to trust in the guarantees Rust makes. However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing catastrophic can occur. The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.  

---

#### * `RefCell<T>` is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

---

## review of reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; - `RefCell<T>` allows immutable or mutable borrows checked at runtime.

Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.  

__Mutating the value inside an immutable value is the interior mutability pattern__. How is it possible?

---

## Interior Mutability: A Mutable Borrow to an Immutable Value

Rust's borrowing rules enforce that when you have an immutable value, you can’t borrow it mutably:

```rust
// This code does not compile!
fn main() {
    let x = 5;
    let y = &mut x;
}
```

```
error[E0596]: cannot borrow immutable local variable `x` as mutable
 --> src/main.rs:3:18
  |
2 |     let x = 5;
  |         - consider changing this to `mut x`
3 |     let y = &mut x;
  |                  ^ cannot borrow mutably
```

Sometimes a value needs to be mutated in its methods (within itself?), but appear immutable to other code.  

Code outside the value’s methods would not be able to mutate the value.  

#### with `RefCell<T>` 

- one way to have interior mutability
- `RefCell<T>` doesn’t get around the borrowing rules completely: 
  - the borrow checker in the compiler allows this interior mutability, 
  - the borrowing rules are checked at runtime instead. 
- If you violate the rules, you’ll get a panic! instead of a compiler error.

### A practical example where we can use `RefCell<T>` to mutate an immutable value and why that is useful

#### A test double is:

- general programming concept for a type used in place of another type during testing
- Mock objects are specific types of test doubles to record what happens during a test,
- so you can assert that the correct actions took place.

---

> Rust doesn’t have objects in the same sense as other languages have objects, and Rust doesn’t have mock object functionality built into the standard library as some other languages do. However, you can definitely create a struct that will serve the same purposes as a mock object.

#### a concrete example

- Let's track a value against a max value,
- and send messages based on how close it is to the max

Imagine a library to keep track of a user’s quota for the number of API calls they’re allowed to make, that does the above ^^  

- only provide the functionality of tracking how close to max a value is and what messages should be sent and when
- Applications that use our library will be expected to provide the mechanism for sending the messages: 
- the message could be in the application, send an email, send a text message, et c
- we need something that implements a trait we’ll provide called `Messenger`

```rust
// interface with send(), Messenger trait
pub trait Messenger {
    fn send(&self, msg: &str);
}

// uh oh lifetimes again haha -- LimitTracker is
// instantiated with a ref to a type that impl Messenger
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize, // could be called current_value
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        // sets value arg onto &mut self
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        // self.messenger.send() is called for alerts
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send(
               "Urgent warning: You've used up over 90% of your quota!",
            );
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

- the `Messenger` trait has a method called `send` that takes an immutable reference to self and the text of the message, __this is the interface our mock object needs to have__
- we want to test the behavior of the `set_value` method on the `LimitTracker`
- We can change what we pass in for the `value` parameter, but `set_value` doesn’t return anything to make assertions on
- We want to be able to say that if we create a `LimitTracker` with something that implements the `Messenger` trait and a particular value for `max`, when we pass different numbers for `value`, the messenger is told to send the appropriate messages.

We need a mock object that:

- instead of sending an email or text message when we call send, will only keep track of the messages it’s told to send
- We can
  - create a new instance of the mock object,
  - create a `LimitTracker` that uses the mock object,
  - call the `set_value` method on `LimitTracker`,
  - and then check that the mock object has the messages we expect.

Here is a naive approach:

```rust
// This code does not compile
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

- defines a `MockMessenger` struct
  - that has a sent_messages field with a Vec of String values to capture what it’s told to send, i.e. `sent_messages`
  - also defined is an associated function `new` to make it convenient to create new `MockMessenger` values
  - We then implement the `Messenger` trait for `MockMessenger` so we can give a `MockMessenger` to a `LimitTracker`.
  - In the definition of the `send` method, we take the message passed in as a parameter and store it in the `MockMessenger` list of `sent_messages`

__In the test, we’re testing what happens when the `LimitTracker` is told to set value to something that is more than 75 percent of the max value.__  

- First, we create a new `MockMessenger`, which will start with an empty list of messages
- Then we create a new `LimitTracker` and give it a ref to the new `MockMessenger` and a `max` value of `100`
- We call the `set_value` method on the `LimitTracker` with a value of `80`, which is more than 75 percent of `100`
- Then we assert that the list of messages that the `MockMessenger` is keeping track of should now have one message in it

However, there’s one problem with this test, as shown here:

```
error[E0596]: cannot borrow immutable field `self.sent_messages` as mutable
  --> src/lib.rs:52:13
   |
51 |         fn send(&self, message: &str) {
   |                 ----- use `&mut self` here to make mutable
52 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ cannot mutably borrow immutable field
```

_why u no compile??_

__PROBLEMS WITH THE ABOVE:__  

- cannot modify the `MockMessenger` to keep track of messages, because `send` method takes an __immutable__ ref to `self`
- cannot take the suggestion from the error text to use `&mut self` instead, because then the signature of `send` wouldn’t match the signature in the `Messenger` trait def

#### This is a situation in which interior mutability can help!

We can store the `sent_messages` within a `RefCell<T>`, and then the `send` message will be able to modify `sent_messages` to store the messages we’ve seen, like the below code:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // value of prop is wrapped in RefCell here, cp other code ^^
        sent_messages: `RefCell<Vec<String>>`, 
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // RefCell used here
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // called with borrow_mut() here
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

__Using `RefCell<T>` to mutate an inner value while the outer value is considered immutable__

The `sent_messages` field is now
- `RefCell<Vec<String>>` instead of `Vec<String>`, also used in `new()` around empty vector

For the implementation of the `send` method:

- the first parameter is still an immutable borrow of `self`, which matches the trait definition
- We call `borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a mutable ref to value inside `RefCell<Vec<String>>`, which is the vector. 
- Then we can call `push` on the mutable ref to the vector to keep track of the messages sent during the test

The last change we have to make is in the assertion: to see how many items are in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an immutable ref to the vector.

## Keeping Track of Borrows at Runtime with `RefCell<T>`

When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively.  

With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`

- The `borrow` method returns the smart pointer type `Ref<T>`, and
- `borrow_mut` returns the smart pointer type `RefMut<T>`.
- Both types implement `Deref`, so we can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. 

- Every time we call `borrow`, the `RefCell<T>` increases its count of how many immutable borrows are active. 
- When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by one. 
- __Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.__  

> If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of `RefCell<T>` will panic at runtime.  

We’re deliberately trying to create two mutable borrows active for the same scope to illustrate that `RefCell<T>` prevents us from doing this at runtime.

```rust
// This code panics!
// Creating two mutable references in the same scope to see that `RefCell<T>` will panic
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

- We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned from `borrow_mut`. 
- Then we create another mutable borrow in the same way in the variable `two_borrow`. 
- This makes two mutable references in the same scope, which isn’t allowed. When we run the tests for our library, the code in Listing 15-23 will compile without any errors, but the test will fail:

```
---- tests::it_sends_an_over_75_percent_warning_message stdout ----
	thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at
'already borrowed: BorrowMutError', src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The code panics with the message already borrowed: `BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing rules at runtime.  

Catching borrowing errors at runtime rather than compile time means that you would find a mistake in your code later in the development process and possibly not until your code was deployed to production. Also, your code would incur a small runtime performance penalty as a result of keeping track of the borrows at runtime rather than compile time. However, using `RefCell<T>` makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while you’re using it in a context where only immutable values are allowed. You can use `RefCell<T>` despite its trade-offs to get more functionality than regular references provide.

## Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

A common way to use `RefCell<T>` is in combination with `Rc<T>`.  

Recall that `Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data.  

If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!

For example, recall the `Cons` where `Rc<T>` is used to allow multiple lists to share ownership of another list.  

Because `Rc<T>` holds only immutable values, we can’t change any of the values in the list once we’ve created them.  

Let’s add in `RefCell<T>` to gain the ability to change the values in the lists. We can use `RefCell<T>` in the `Cons` definition, and modify the value stored in all the lists:

```rust
// Using Rc<RefCell<i32>> to create a List that we can mutate
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 1, we create a value that is an instance of 
    // Rc<RefCell<i32>> and store it in `value`
    let value = Rc::new(RefCell::new(5));

    // 2. then `List` in `a` with a `Cons` variant that holds value. 
    // - clone `value` so both `a` and `value` have ownership of inner `5` value, 
    // instead of transferring ownership `value` -> `a` or having `a` borrow from `value`.

    // 3. then wrap `a` in Rc<T> so lists `b` and `c` can both refer to `a`
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 'value' uses auto dereferencing to deref Rc<T> to inner RefCell<T> value
    // .borrow_mut() returns RefMut<T> smart pointer
    // use deref operator (*) on it and change the inner value
    // += add 10 to value by calling `borrow_mut` on value
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

When we print `a`, `b`, and `c`, all have modified value of 15 rather than 5:

```
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

### Conclusion

Using `RefCell<T>`, we have an outwardly immutable List value.  

But we can use the methods on `RefCell<T>` that provide access to its interior mutability so we can modify our data when we need to.  

__The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures.__  

The standard library has other types that provide interior mutability, such as:

- `Cell<T>`, which is similar except that instead of giving references to the inner value, the value is copied in and out of the `Cell<T>`.
- There’s also `Mutex<T>`, which offers interior mutability that’s safe to use across threads.
  