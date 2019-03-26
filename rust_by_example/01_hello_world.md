# Hello World

## Print to the Console

```rust
// let's call this world.rs

fn main() {
    println!("Hello World!");
}
```

- `println!` is a macro function that prints to the console.

You can generate a binary using the Rust compiler from the command line, like this:

```
$ rustc world.rs
```

This will produce an executable binary in the same directory, `world`, to be run like this:

```
$ ./world   // output => Hello World!
```

## Comments

They look like this: 

```rust
// a comment
fn normal_fn() {
    136 // this is a second comment
}
```

## Formatted Print

Printing is handled by a series of macros defined in std::fmt some of which include:

- `format!`: write formatted text to String -- this is used for assigning a String to a variable

- `print!`: same as format! but the text is printed to the console (io::stdout).
- `eprint!`: same as format! but the text is printed to the standard error - (io::stderr).

- `println!`: same as print! but a newline is appended.
- `eprintln!`: same as eprint!but a newline is appended.

```rust
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // I like this one!
    
    // As can named arguments. 
    println!(
      "{subject} {verb} {object}",
      object="the lazy dog",
      subject="the quick brown fox",
      verb="jumps over",
    );

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // oh crazy!

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
    
    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
```
