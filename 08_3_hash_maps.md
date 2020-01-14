# Storing Keys with Associated Values in Hash Maps

Technically `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.

## Creating a New Hash Map

```rust
// hash maps are not auto included in the prelude
use std::collection::HashMap;

// type HashMap<String, i32>
let mut scores = HashMap::new();

// the types of K and V are inferred here
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- the least used of structures in chapter 8
- no `macro` exists for creating it
- data in hash maps is stored on the heap, just like vectors

### Alternate Method using `collect`

```rust
use std::collections::HashMap;

// keys
let teams = vec![String::from("Blue"), String::from("Yellow")];
// values
let initial_scores = vec![10, 50];

// hash map is created here
// the underscores allow Rust to infer the types by using them
let scores: HashMap<_, _> = teams.iter()
    .zip(initial_scores.iter())
    .collect();
```

## Hash Maps and Ownership

- for types that implement the `Copy` trait, like `i32`, the values are copied into the hash map
- for `owned` values like `String`, the values are moved and the hash map will be the owner of the values

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

// notice the `mut` keyword
let mut map = HashMap::new();

// after the next line, `field_name` and `field_value`
// are invalid within the scope and are now owned by `map`
map.insert(field_name, field_value);
```

## Accessing Values in a Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");

// notice the &team_name
let score = scores.get(&team_name);
```

`let score = scores.get(&team_name);` will yield a result wrapped in `Some` because `get` returns `Option<&V>`. If no value exists, `get` will return `None`. `Option` will need to handled as described in Chapter 6.

### Iteration over Key/Value Pairs

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

## Updating a Hash Map

### Overwriting a Value (When a Key Already Exists)

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// insert is called 2x on Blue
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // prints {"Blue": 25}
```

### Upserting a Value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// this is new, so the K/V is inserted
scores.insert(String::from("Blue"), 10);

// this checks for the key Yellow, does not find one, inserts the K/V
scores.entry(String::from("Yellow")).or_insert(50);

// this is not new, Blue already exists, so no update occurs
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores); // prints {"Yellow": 50, "Blue": 10}
```

### Updating a Value Based on Existing Value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

// text.split_whitespace creates ["hello", "world", "wonderful", "world"]
for word in text.split_whitespace() {
    // upserts `word` with count, or 0
    let count = map.entry(word).or_insert(0);
    // increments by 1
    *count += 1;
}

println!("{:?}", map); // prints {"world": 2, "hello": 1, "wonderful": 1}
```

- `or_insert` returns a mutable reference `&mut V` to the value for this key, stored in a mutable reference in the `count` variable, so in order to assign to that value, we must first dereference count using the asterisk `*`. The mutable reference goes out of scope at the end of the `for` loop, so all of these changes are safe and allowed by the borrowing rules.
