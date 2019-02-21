# Data Types

- statically typed
- compiler can usually infer the type based on the value, unless several types are possible, for example when converting a string into an integer type

## Scalar Types

- represents a single value
- four primary scalar types:
    1. integers
    2. floating-point numbers
    3. Booleans
    4. characters

### 1. Integers

- a number without a fractional component
- signed and unsigned integers

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8 bit   | i8     | u8       |
| 16 bit  | i16    | u16      |
| 32 bit  | i32    | u32      |
| 64 bit  | i64    | u64      |
| 128 bit | i128   | u128     |
| arch    | isize  | usize    |

- each signed type can hold -(2^(n-1)) => 2^(n-1)-1

for example: `i8`, with `n=8`

```math
-(2^(8-1)) => 2^(8-1)-1

-128 => 127
```

- the isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture. This is only really important if you are indexing a huge collection.

- `i32` is the default type that Rust chooses

#### Integer Overflow

- Rust checks for integer overflow only in debug and causes a `panic`, but not in production. In prod, Rust will use "two's complement wrapping"

### 2. Floating-Point Types

- decimal numbers, like these:

```rust
let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

- Rust supports normal math operations

### 3. Booleans

- `true` and `false`
- one byte in size

### 4. Character Type

- `char` type represents a Unicode Scalar Value
- specified with `''` single quotes, as opposed to string literals which use `""` double quotes

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

## Compound Types

- there are two primitive compound types
    1. tuple
    2. array

### 1. Tuple

- a general way of collecting values of different types into a single compound type
- have a fixed length
- created like this:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- tup is considered a single element
- to access individual values, you can destructure (just like ES6)

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

- or you can use `.` dot notation to access values directly from the tuple, like this:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### 2. Array

- array items must be of the same data type
- have a fixed length (vectors do not)
- useful when you want the data on the stack and when you want to ensure a certain length of array

```rust
let months = [
    "January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December",
];
```

- arrays have an interesting type; it looks like this: [type; number_of_items]. For example:

```rust
// type: i32, length: 5
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- accessed with `[n]` with n being the index, just like in javascript
- invalid array element access DOES NOT produce a compile error, BUT DOES produces a runtime error
