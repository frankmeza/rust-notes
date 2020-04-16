# Running Code on Cleanup with the `Drop` Trait

- the second trait important to the smart pointer pattern is `Drop`
- `Drop` lets you customize what happens when a value is about to go out of scope
- you can provide an implementation for the `Drop` trait on any type, and the code you specify can be used to release resources like files or network connections.  

`Drop` is introduced in the context of smart pointers because the functionality of the `Drop` trait is almost always used when implementing a smart pointer.  

- ex., `Box<T>` customizes `Drop` to deallocate the space on the heap that the box points to.

> In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!

### Implementing the `Drop` Trait

The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to self.  

The `CustomSmartPointer` struct whose only custom functionality is that it will print `"Dropping CustomSmartPointer!"` when the instance goes out of scope to see when Rust runs the `drop` function.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // 3. executed third print statement, via `drop` method on CustomSmartPointer
    let c = CustomSmartPointer { data: String::from("my stuff") };
    
    // 2. executed second print statement, via `drop` method on CustomSmartPointer
    let d = CustomSmartPointer { data: String::from("other stuff") };
    
    // 1. executed first print statement
    println!("CustomSmartPointers created."); 
}

// the above will print, in order:
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!
```

#### The `Drop` trait 

- is included in the prelude, so we don’t need to bring it into scope. 
- is implemented on `CustomSmartPointer` and provide an implementation for the `drop` method that calls `println!`
- the body of the `drop` function is where you would place any logic that you wanted to run when an instance of your type goes out of scope, like printing text here.
- two instances of `CustomSmartPointer` are created and then print CustomSmartPointers created. 
- at end of scope, our instances of `CustomSmartPointer` will go out of scope, and the `drop` method is called implicitly by Rust.

Rust automatically called `drop` for us when our instances went out of scope, calling the code we specified. Variables are dropped in the reverse order of their creation, so `d` was dropped before `c`. This example gives you a visual guide to how the `drop` method works; usually you would specify the cleanup code that your type needs to run rather than a print message.

## Dropping a Value Early with `std::mem::drop`

Unfortunately, it’s not straightforward to disable the automatic `drop` functionality.  

Disabling `drop` isn’t usually necessary; the whole point of the `Drop` trait is that it’s taken care of automatically. Occasionally, you must clean up a value early. One example is when using smart pointers that manage locks: you might want to force the `drop` method that releases the lock so that other code in the same scope can acquire the lock.  

Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.  

```rust
// does not compile

fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");

    c.drop(); // not allowed!
    println!("CustomSmartPointer dropped before the end of main.");
}
```

```
error[E0040]: explicit use of destructor method
  --> src/main.rs:14:7
   |
14 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
```

We cannot explicitly call `drop`. A destructor cleans up an instance, and is analogous to a constructor, which creates an instance. The `drop` function in Rust is one particular destructor.  

Rust doesn’t let us call `drop` explicitly because Rust would still automatically call `drop` on the value at the end of main.  

__This would be a double free error because Rust would be trying to clean up the same value twice.__

The automatic insertion of `drop` when a value goes out of scope cannot be disabled, nor can we call the `drop` method explicitly. So, if we need to force a value to be cleaned up early, we can use the `std::mem::drop` function.

The `std::mem::drop` function is different from the `drop` method in the `Drop` trait.  

- it's called by passing the value we want to force to be dropped early as an argument, like this:

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created."); // 1. printed first

    // 2. message in `drop()` printed from `c.drop()` method
    drop(c); // compare to above example's `c.drop();`
    println!("CustomSmartPointer dropped before the end of main."); // 3. printed third
}

// the above will print, in order:
// 1. CustomSmartPointer created.
// 2. Dropping CustomSmartPointer with data `some data`!
// 3. CustomSmartPointer dropped before the end of main.
```

- code specified in a `Drop` trait implementation makes cleanup convenient and safe: ex., you could use it to create your own memory allocator! 
- with the `Drop` trait and Rust’s ownership system, Rust cleans up allocation automatically.
- accidentally cleaning up values still in use is no problem: the ownership system that makes sure references are always valid also ensures that `drop` gets called only once when the value is no longer being used.
