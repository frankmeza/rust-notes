# `mod` and the Filesystem

## Module Definitions

- defines a module named `network` using the `mod` keyword, same as `client`

```rust
// lib.rs

mod network {
    fn connect() {}
}

mod client {
    fn connect() {}
}
```

Each `connect` function is called with its namespace:

```rust
network::connect()

client::connect()
```

- the namespacing prevents conflict on `connect`

```
// the structure of modules

communicator
 ├── network
 └── client
```

### Another Example


```rust
// lib.rs

mod network {
    fn connect() {}

    mod client {
        fn connect() {}
    }
}

```

These don't conflict either:

```rust
network::connect()

network::client::connect()
```

```
communicator
 └── network
     └── client
```

### Splitting Up Modules Into Separate Files

```rust
// lib.rs

mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

> We’re still declaring the client module here, but by replacing the block with a semicolon, we’re telling Rust to look in another location for the code defined within the scope of the client module. In other words, the line mod client; means this:

Now create a file `client.rs` . Inside of this file is this:

```rust
// client.rs

fn connect() {}
```

We do not need to redeclare `mod client` in here. Rust already knows that this is the resource needed in the example above.

Now let's clean up further and create `network.rs` into which we put:

```rust
// network.rs

fn connect() {}

// server is a submodule of network, so it
// must stay in the same file with its parent
mod server {
    fn connect() {}
}
```

## Rules of Module Filesystems

> Let’s summarize the rules of modules with regard to files:
If a module named `foo` has no submodules, you should put the declarations for `foo` in a file named `foo.rs`.
If a module named `foo` does have submodules, you should put the declarations for `foo` in a file named `foo/mod.rs`.
These rules apply recursively, so if a module named `foo` has a submodule named `bar` and `bar` does not have submodules, you should have the following files in your src directory:

```
└── foo
    ├── bar.rs (contains the declarations in `foo::bar`)
    └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

The modules should be declared in their parent module’s file using the mod keyword.