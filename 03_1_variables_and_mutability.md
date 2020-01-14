# Variables and Mutability

## Variables

- `let` is used to declare variables, which are immutable by default
- `let mut` is used to declare mutable variables

### RIGHT ✅

```rust
let mut x = 5;
x = 6;
```

### WRONG 🚫

```rust
let x = 5;  // x is not marked with `mut`
x = 6;      // so it cannot be reassigned
```

## Constants

- defined like this, and are always immutable:

```rust
const IS_LIT = true;
```

## Variable Shadowing

- Variable shadowing is allowed on the idea of using `x` as an accumulator when you have to do that.
- The shadowing itself is actually an overwriting of old variable data.
- **Remember that because you ARE creating a new variable _over_ the old one, you can change the data type too.**
    - This seems out of place in Rust, but whatever. Just don't it unless you know and will 100% remember, always and forever.

### RIGHT ✅

```rust
let x = 5;
let x = x + 1;
```

```rust
let spaces = "  ";
let spaces = spaces.len();
```

### WRONG 🚫

- this does not work with mutable variables

```rust
let mut spaces = "  ";
spaces = spaces.len();
```
