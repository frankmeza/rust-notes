# Recoverable Errors with `Result`

Most errors do not really require the program to crash.

- can use `Result` to handle errors

```rust
// T, E are generic type parameters -
// more about this in chapter 10
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `T` will be returned in the success case
- `E` will be returned in the error case

```rust
use std::fs:File;

fn main() {
    // returns Result, defined above
    let f = File::open("hello");
}
```

If you try to type `f` as anything other than `Result`, the compiler will let you know that that `f` here is `std::result::Result<std::fs::File, std::io::Error>`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err)
        }
    }
}
```

- this is fairly straightforward error handling, it feels pretty similar to most error handling in TS/JS

## Matching on Different Errors

The above code will `panic!` no matter what the error is. We can write code to distinguish between the different error types and handle them differently.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // returns io::Error, with #kind()
        Err(err) => match error.kind() {
            // "if 'not found' is the returned error from File::open("hello.txt"),
            // then File::create it. If that works, return that Result, else `panic!`
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("There was an error creating the file: {:?}", e),
            },
            // I think that this name is arbitrary
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        }
    }
}
```

## Shortcuts for Panic on Error: `unwrap` and `expect`

- using `match` can become verbose
- if `Result` returns the `Ok` variant, we can use `unwrap` to access the `Ok` value

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

- The above code does not handle the `Err` and so will `panic!`.
- `expect` can be used similarly to `unwrap`, but to handle `Err` instead of `Ok`

```rust
use std::fs::File;

fm main() {
    let f = File::open("hello.txt").expect("some kind of error here");
}
```

- This is useful in debugging exactly where an error occurs.

## Propagating Errors

- this is the technical term for passing an error to a function's caller, instead of only having the option of handling it within that function.

```rust
// #1 NAIVE VERSION

use std::io;
use std::io::Read;
use std::fs::File;

// returns Result to its caller
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    // f will either be the file, or the error.
    // this block handles the variants of Result from File::open()
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // the f is stringified and returned from this function
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

- if this code succeeds and there are no errors, `String` will be returned as the `T` value from `Result<T, E>`. Else, `io::Error` will be returned as the `E`.

## A Shortcut for Propagating Errors: the `?` Operator

```rust
// #2 PRODUCTION VERSION

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt");
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}
```

- `f.read_to_string(&mut s)?;` is the crux of this section
- the `?` immediately propagates the error to the calling code, as if we wrote code to explicitly return the error

```rust
//  #3 PRO VERSION, with method chaining

use std::io;
use std::io::Read;
use std::io::File;

fn read_username_from_file() -> Result<String, io::Error> {
    // creates the String to be returned
    let mut s = String::new();

    // File::open() is checked for errors with ?, so is `read_to_string`
    File::open("hello.txt")?.read_to_string(&mut s)?;
    // if all goes well, `s` is returned as `String`
    Ok(s)
}
```

## The `?` Operator Can Only Be Used with Functions That Return `Result`

- Otherwise, `Err` would have to explicitly be given as the return type, and you will have to use the `match` operator to handle any errors within the code.
