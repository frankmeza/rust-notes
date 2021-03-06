# Packages and Crates for Making Libraries and Executables

- a crate is a binary, or lib
- the crate root is a source file that knows how to build a crate
- each package has a Cargo.toml that describes how to build 1+ crates. At most one crate in a package can be a library.

So on `cargo new`, we’re creating a package:

```bash
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

- cargo created `Cargo.toml` -> we now have a package.

> "Cargo’s conventions are that if you have a `src` directory containing main.rs in the same directory as a package’s Cargo.toml, Cargo knows this package contains a binary crate with the same name as the package, and src/main.rs is its crate root."  

> "Another convention of Cargo’s is that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. The crate root files are passed by Cargo to rustc to actually build the library or binary."  

> "A package can contain zero or one library crates and as many binary crates as you’d like. There must be at least one crate (either a library or a binary) in a package."  

> "If a package contains both src/main.rs and src/lib.rs, then it has two crates: a library and a binary, both with the same name. If we only had one of the two, the package would have either a single library or binary crate. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate."  
