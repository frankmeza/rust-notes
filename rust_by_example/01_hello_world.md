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
