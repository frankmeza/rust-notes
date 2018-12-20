// When I was going through Chapter 7, there was suddenly a large section
// which swallowed some of the concepts from the previous section.
// I just cover the material as it is, when I get to it.

# The Module System to Control Scope and Privacy

This is an overview of:

- Modules, a way to organize code and control the privacy of paths
- Paths, a way to name items
- use a keyword to bring a path into scope
- `pub`, a keyword to make items public
- Renaming items when bringing them into scope with the `as` keyword
- Using external packages
- Nested paths to clean up large use lists
- Using the glob operator to bring everything in a module into - scope
- How to split modules into individual files

## Modules

```rust
mod sound {
    fn guitar() {
        // Function body code goes here
    }
}

fn main() {
}
```

- 2 functions, `guitar` and `main`
- `guitar` is within a module `sound`

### Another Example

```rust
mod sound {
    mod instrument {
        mod woodwind {
            fn clarinet() {
                // Function body code goes here
            }
        }
    }

    mod voice {
    }
}

fn main() {
}
```

- 2 functions `clarinet` and `main`
- `clarinet` is within some nested modules

## Paths for Referring to an Item in the Module Tree

- to call clarinet in the previous example, we would do

```rust
fn main() {
    // absolute path
    crate::sound::instrument::woodwind::clarinet()

    // relative path
    sound::instrument::woodwind::clarinet()
}
```

## Modules as the Privacy Boundary

Calling the above would throw an error because modules default to private between each other.

> All items (functions, methods, structs, enums, modules, annd constants) are private by default.
> You can use the pub keyword to make an item public.
> You aren’t allowed to use private code defined in modules that are children of the current module.
> You are allowed to use any code defined in ancestor modules or the current module.

```rust
// With `instrument` and `woodwind` marked with `pub`,
// the function calls in `main()` will compile

mod sound {
    pub mod instrument {
            pub woodwind {

            pub fn clarinet() {
                // Function body code goes here
            }
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

## Starting Relative Paths with super

- you can construct relative paths with `super`
- it's similar to "../" in TS/JS

```rust
mod instrument {
    fn clarinet() {
        // goes to the parent level and finds breathe_in()
        super::breathe_in();
    }
}

fn breathe_in() {
    // Function body code goes here
}
```

In the above, we could have called `crate::breathe_in` but if `instrument` and `breathe_in` are not in the root `crate` directory as they are now, we would have to make a change.

## Using pub with Structs and Enums

### Struct

```rust
mod plant {
    pub struct Vegetable {
        pub name: String, // this is marked with `pub`
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
```

`Vegetable.name` is public, so it's readable and writable, `id` however is private.

### Enum

With enums however, all variants of an enum are made public, unlike the fields of a struct.

```rust
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
```

## The `use` Keyword to Bring Paths into a Scope

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

// this is a symbolic link that puts `instrument`
// in the scope of the `use` statement
use crate::sound::instrument;
// absolute path ^^
// relative path vv
use self::sound::instrument;

fn main() {
    // this function doesn't need to know
    // about sound::instrument explicitly
    instrument::clarinet();
}
```

## Idiomatic use Paths for Functions vs. Other Items

### idiomatic Rust for Functions

```rust
use grandparent::parent;

fn main() {
    // referenced from parent
    parent::function();
}
```

### NOT idiomatic Rust for Functions

```rust
use grandparent::parent::function;

fn main() {
    function();
}
```

### idiomatic Rust for Structs, Enums, and other Items

```rust
// referenced directly
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

### NOT idiomatic Rust

```rust
use std::collections;

fn main() {
    let mut map = collections::HashMap::new();
    map.insert(1, 2);
}
```

If two items are brought into scope with the same name, one of them must be changed, or aliased.

## Renaming Types Brought Into Scope with the as Keyword

### Either of these is idiomatic for aliasing an item

```rust
use std::fmt::Result;
use std::io::Result as IoResult; // use `as` keyword

fn function1() -> Result {
}

// references alias name
fn function2() -> IoResult<()> {
}
```

```rust

use std::fmt;
use std::io;

// reference parent
fn function1() -> fmt::Result {
}
fn function2() -> io::Result<()> {
}
```

## Re-exporting Names with pub use
