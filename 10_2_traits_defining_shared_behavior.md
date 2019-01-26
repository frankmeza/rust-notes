# Traits: Defining Shared Behavior

- a way to let the Rust compiler know about shared functionality between different types of structs
- similar to `interface` in other coding languages

## Defining a Trait

- imagine a `NewsArticle` and a `Tweet` :
    - have similarities and differences in the way that they contain data
- imagine a media aggregator library that displays summaries of data for both. We need to have a shared `Summary` trait with a function `summarize`

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- define a trait using the keyword
- `Summary` is the trait name
- inside the block are the shared method signatures of the structs that implement `Summary`
- the compiler will enforce that any type with the Summary trait must have a `summarize` method that implements the shared function signature
- each type can implement the function signature in its own way

## Implementing a Trait on a Type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}))", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### The Above Allows Us To Do This

```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
// => 1 new tweet: horse_ebooks: of course, as you probably already know, people.
```

### Scope of Implementing Traits

- in order to use an external library's Trait, you will have to bring that Trait into scope of your code, with this: `use aggregator::Summary;`
- this also needs to be marked public with `pub`
- Rust will not allow you to define external traits on external types, due to `coherence` and the `orphan rule` due to the parent type being missing

## Default Implementations
