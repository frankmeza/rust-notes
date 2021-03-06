# Closures: Anonymous Functions that Can Capture Their Environment

- Rust's closures allow you to access and capture the variables in one scope from another scope. They can be created in one place and then used elsewhere.

## Creating an Abstraction of Behavior with Closures

### A Hypothetical Expensive Calculation

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value ,
        simulated_random_number,
    );
}
```

- `simulated_expensive_calculation` is in too many places...

## Refactoring Using Functions

- the result of the `simulated_expensive_calculation` can be stored in a variable at the top of the fn body, and then the results can be passed into the active arm of the code

## Refactoring with Closures to Store Code

```rust
let expensive_closure = |num| {
    println!("calculating slowly..");
    thread::sleep(Duration::from_secs(2));
    num
}
```

A closure:  

- encapsolates behavior itself, instead of a value. 
- This is how similar behavior can be spread across different scopes. 
- It is a function whose parts have distinct relationships to each other, and has shared behavior with nearby code which could be encapsulated with a variable. 
- It's clean looking though  

After seeing this comparison:

```rust
fn add_one_v1    (x: u32) -> u32 { x + 1 } // regular function syntax
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

## Closure Type Inference and Annotation

Closures do not require to be typed, like other functions. Closures are not public facing. However, in the pursuit of explicitness and strictness, a closure with annotations like `add_one_v2`

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

- say that a `String` first enters the closure and walks out without error, the next `i32` will not compile. Not cool, dude. Blame the compiler inference. It locks its first data type as the unique one used within that closure.  

## Storing Closures Using Generic Parameters and the Fn Traits

### My Own Thoughts About It All, After The First Read

__It's a struct that holds a closure, so that it can be passed around instead of the closure itself__

__It's like a JS object that has a function on it as part of a key/value pair__

__It's like a pointer to a function...???...!!!...???__

__"I'm going to give you this function that I'm going to run inside of your function, so that you can give me the variables that I need to run my function."__

### My Own Thoughts About It All, After Some Time Has Passed

__It's like passing around an empty gun so anyone can shoot their own bullets.__  

__It's like a waiter that moves around between callers as they need to order data processed.__  

__A closure is like easy bake functions available to use by code local to it.__  

In Rust, a `struct` can be created to hold the closure. In order to do that, a type must be specified for the closure to give to the `struct`.  

> Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different. To define structs, enums, or function parameters that use closures, we use generics and trait bounds, as we discussed in Chapter 10.

All closures implement at least one of the following:

- `Fn`
- `FnMut`
- `FnOnce`

```rust
// struct Cacher over a generic type, where 
// that type implements fn signature `Fn :: u32 -> u32`
struct Cacher<T>
    where T: Fn(u32) -> u32 // 2. the trait bound on T is that of a closure (Fn, FnMut, FnOnce)
{
    calculation: T, // 1. the calc field of the generic type T
    value: Option<u32>,
}
```

1. The Cacher struct has a calculation field of the generic type T. 
2. The trait bounds on T specify that it’s a closure by using the Fn trait.
3. |When in Use, not in definition| -> Any closure we want to store in the calculation field must have 
  * one u32 parameter (in parens) and must return a u32 (after ->).


```rust
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None, // 1. Option<u32>, but 2. right now None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.calculation)(arg); // 3. value is set
                self.value = Some(v);
                v
            },
            Some(v) => v, // 4. value is returned if already set
        }
    }
}
```

1. The value field is of type Option<u32>.
2. Before we execute the closure, value will be None. 
3. When code using a Cacher asks for the result of the closure, the Cacher will execute the closure at that time and store the result within a Some variant in the value field. 
4. Then if the code asks for the result of the closure again, instead of executing the closure again, the Cacher will return the result held in the Some variant.

```typescript
// possible pseudo-typescript implementation?

type Closure = (number) => number
type Value = number | null

interface Cacher {
    closure: Closure
    value: Value
}

class Cacher<calculation: Closure>:  {
    constructor(T) {
        this.calculation = calculation 
        this.value = null
    }

    value = (num: number): Value => {
        if (!this.value) {
            this.value = this.calculation(num)   
        }

        return this.value
    }
}

const someFn: Closure = (n: number) => n * n
const anonClosure = new Cacher(someFn)
```

## Limitations of the Cacher Implementation

- the struct assumes that it will always get the same value for its argument

The following test will fail:

```rust
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

- the fix is to have `Cacher` hold a hash map instead of a single value.
  - the key will be the `arg` passed into the calculation
  - the value will be the result of calling the closure on that key
  - instead of checking for `Some` on the single value, the key will be searched for. If not present, `Cacher` will call the closure and save the result as the value for that key

## Capturing the Environment with Closures

Closures have a capability that regular functions do not have: the ability to capture their environment and access variables in that scope.  

```rust
fn main() {
    // define x
    let x = 4;

    // create a comparison closure
    let equal_to_x = |z| z == x;

    // define y
    let y = 4;

    // equal_to_x captures `x` and its value,
    // then compares its arg `y` to `x`
    assert!(equal_to_x(y));
}
```

### Points about the Compiler and Memory

- in capturing the environment, a closure uses memory to do so.
- it's best if normal functions do not do this so that we do not incur this overhead cost in memory.

- Closures capture values from their environment in three ways that map to the ways functions take parameters:
  1.  taking ownership, `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The `Once` part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

  2. borrowing mutably, `FnMut` can change the environment because it mutably borrows values.

  3. and borrowing immutably, `Fn` borrows values from the environment immutably.

- When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment. 
- All closures implement `FnOnce` because they can all be called at least once. 
- Closures that don’t move the captured variables also implement `FnMut`, 
- and closures that don’t need mutable access to the captured variables also implement `Fn`. 
- the above example `equal_to_x` closure borrows x immutably (so equal_to_x has the Fn trait) because the body of the closure only needs to read the value in x.
