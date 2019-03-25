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
