# Defining an Enum

- there are some cases when an enum is more appropriate than using a struct
- IP4 and IP6 for instance are exclusive variants of the same thing, an IP address

```rust
enum IpAddrKind {
    V4,     // V4 and V6 are *variants* of the enum IpAddrKind
    V6,
}
```

- an enum *looks* to be similar to Typescript's type, ex. `type ipAddrKind = V4 | V6`

## Enum Values

Enums are created like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- both of these values are of the same kind, `IpAddrKind`
- so they can be passed into the same functions

```rust
fn route(ip_type: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

An enum used in conjunction with a `struct` might look like this:

```rust
enum IpAddrKind {
    V4,                     // <- variants are declared
    V6,
}

struct IpAddr {
    kind: IpAddrKind,       // <- the enum is used as a type
    address: String,        // within a struct
}

let home = IpAddr {         // <- an instance is created
    kind: IpAddrKind::V4,   // <- the enum is used as a type
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Equivalent code in Typescript might be:

```typescript
type IpAddrKind = V4 | V6

interface IpAddr {
    readonly kind: IpAddrKind
    readonly address: string
}

const home: IpAddr = {
    kind: V4,
    address: "127.0.0.1",
}

const loopback: IpAddr = {
    kind: V6,
    address: "::1",
}
```

The above code can be shortened up like this:

```rust
enum IpAddr {
    V4(String), // a String is immediately passed
    V6(String), // into the variant when created
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

This second code snippet renders the first snippet's `struct IpAddr` unneeded

### An Aside About Storing V4 and V6 IP Addresses

- the Rust standard library has a definition to do just this:

```rust
struct Ipv4Addr {
    // code
}

struct Ipv6Addr {
    // code
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

- any kind of data is valid within an enum variant, even other enums

### A More Complex Example of Enum

```rust
// once again, an enum defines its variant types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
```

There are four variants with different types:

- `Quit` has no data associated with it at all.
- `Move` includes an anonymous struct inside it.
- `Write` includes a single String.
- `ChangeColor` includes three i32 values.

Several `struct`s can be used to encapsulate the data structure in the enum above:

```rust
struct QuitMessage; // unit struct

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct

struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But done this way, it becomes more difficult to write *what-would-be* **enum methods**. Another similarity of `struct`s and `enum`s is that methods can be defined on them in `impl` blocks:

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();   // String::from("hello") is the value on `m` that is going into `call()`
```

## The Option Enum and Its Advantages Over Null Values

`Option` is an enum defined by the standard library. It handles the possibility of a `null` value, when a value could be something or nothing. `null` has been at the heart of many many bugs during the past 40 years.

Rust does not have nulls, but rather an enum that encodes the concept of a value being present or absent. This enum is `Option<T>`, and it is defined by the standard library as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- the `<T>` syntax is a feature of Rust called a generic type parameter
- for now, `<T>` means that the `Some` variant of `Option` can hold one piece of any data type, ex:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

- when creating a variable with value `None`, we must define it as `Option` with a data type, as Rust cannot infer the data type of a `None` value

### Why This Is Better Than Just Having Null

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

// this will cause an error that `i8` and `Option<i8>`
// cannot be added together
let sum = x + y;
```

- an `Option<T>` must be converted into `<T>` before you can do `<T>` things with it

How does one do this conversion?

> In general, in order to use an Option<T> value, you want to have code that will handle each variant. You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner type `<T>`. You want some other code to run if you have a None value, and that code doesn’t have a T value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
