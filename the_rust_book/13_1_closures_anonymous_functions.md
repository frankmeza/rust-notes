# Closures: Anonymous Functions that Can Capture Their Environment

- Rust's closures allow you to access and capture the variables in one scope from another scope. They can be created in one place and then used elsewhere

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
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

it almost appears as if closures are a kind of _dialect_ of functions.  

The second one, 

```rust
let add_one_v2 = |x: u32| -> u32 { x + 1 };
```

looks just like

```typescript
const addOneV2 = (n: number): number => { n + 1 }
```

## Closure Type Inference and Annotation

Closures do not require to be typed, like other functions. Closures are not public facing. However, in the pursuit of explicitness and strictness, a closure with annotations like `add_one_f2`

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

- say that a `String` first enters the closure and walks out without error, the next `i32` will not compile. Not cool, dude. Blame the compiler inferrence. It locks its first data type as the unique one used within that closure.  

## Storing Closures Using Generic Parameters and the Fn Traits

### reread this again

__It's a struct that holds a closure, so that it can be passed around instead of the closure itself__

__It's like a JS object that has a function on it as part of a key/value pair__

__It's like a pointer to a function...???...!!!...???__

"I'm going to give you this function that I'm going to run inside of your function, so that you can five me the variables that I need to run my function."