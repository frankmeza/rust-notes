# Variables and Mutability

## Variables

-   `let` is used to declare variables, which are immutable by default
-   `let mut` is used to declare mutable variables

### RIGHT

```rust
let mut x = 5;
x = 6;
```

### WRONG

```rust
let x = 5;
x = 6;
```

## Constants

-   defined like this, and are always immutable:

```rust
const IS_LIT = true;
```

## Variable Shadowing

-   variable shadowing is allowed on the idea of using x as an accumulator when you have to do that
-   remember that because you ARE creating a new variable _over_ the old one, you can change the data type too
-   is overwriting of old variable data

### Right

```rust
let x = 5;
let x = x + 1;
```

```rust
let spaces = "  ";
let spaces = spaces.len();
```

### WRONG

-   this does not work with mutable variables

```rust
let mut spaces = "  ";
spaces = spaces.len();
```
