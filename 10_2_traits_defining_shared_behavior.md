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

- it's useful to have a default implementation for methods within a `Trait`

To set a default impl:

```rust
pub trait Summary {
    // given a block, instead of just `;`
    fn summarize(&self) -> String {
        String::from("(Read more..."))
    }
}

// to use the default impl for a type,
// give the `impl` block no function body
impl Summary for NewsArticle {}
```

### More about Default Implementations

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

- default impl methods can call other methods within the `Trait` that do NOT have a default implementation, i.e. ^^ default `summarize` can call the individual implementation of Trait methods, ex. `summarize_author`, like this:

```rust
// unique impl method for summarize_author on Tweet
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// given this tweet struct,
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

// we can call this `summarize` on tweet
println!("1 new tweet: {}", tweet.summarize());
```

## Traits as Arguments (functions that receive a Trait as a data type)

- we can also define functions that receive a data type that have a given Trait

```rust
pub fn notify(item: impl Summary) {
    // in here, item is a type that implements the Summary trait,
    // so we can call any method on item that exists within the trait
    println!("Breaking news! {}", item.summarize())
}
```

### Trait bounds

The above code snippet is syntactic sugar for the following:

```rust
// "...notify is a function generic over type Summary, which receives that type"
pun notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}
```

- the above is explicitly naming `Summary` as the argument type, and so using other data types will cause the code to not compile.

#### When You Should Use This More Verbose Form

When complexity and multiple types are present, opt for the more verbose form.

For example:

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {}
```

- `notify` accepts two possible types, both of which impl `Summary`.

> This would work well if item1 and item2 were allowed to have diferent types (as long as both implement Summary). But what if you wanted to force both to have the exact same type? That is only possible if you use a trait bound:

```rust
pub fn notify<T: Summary>(item1: T, item2: T) {}
```

- in the above, `notify` is generic over only ONE type which implements `Summary`, and so we use the verbose form to make that explicit.

So we could call this:

```rust
notify(tweet1, tweet2);
```

or this:

```rust
notify(news_article1, news_article2);
```

but not these:

```rust
notify(tweet, news_article);

notify(news_article, tweet);
```

### Specify multiple traits with `+`

It's possible to concat two traits as an accepted type for an argument:

```rust
// as a trait bound
pub fn notify(item: impl Summary + Display) {}

// more verbose with explicit generic type
pub fn notify<T: Summary + Display>(item: T) {
```

In the above, `item` must implement both `Summary` and `Display` traits.

### `where` clauses for clearer code (and readability)

Rust also has a `where` keyword, which allows you to write the above as this, improving clarity of the function signature:

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // function body here
}
```

*Read in plain language*: `some_function` is generic over types T and U, where `T` implements traits Display and Clone, and where `U` implements traits Clone and Debug, and returns `i32`.

## Returning Traits