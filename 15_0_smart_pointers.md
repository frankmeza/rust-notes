# Smart Pointers

A _pointer_ is a general concept of a variable that contains an address of data in memory. It __points__ at the real data (location). Most commonly, Rust uses `&` to denote a reference, like `&data_name`.  

A _smart pointer_ is a data structure that acts like a pointer but also has additional metadata and abilities, including the reference counting smart pointer type. This allows multiple owners of data, and then cleans up the memory of the data once no owners are found.  

In contrast to Rust's regular reference pointers, smart pointers often own the data they point to.  

(What??)  

### Known Examples of Smart Pointers

`String` and `Vec<T>` are smart pointers. Both these types count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with String ensuring its data will always be valid UTF-8).  

### Implementing Smart Pointers

Smart pointers are usually implemented using `struct`s.  

The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits.  

The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.  

The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope. In this chapter, we’ll discuss both traits and demonstrate why they’re important to smart pointers.

### Smart Pointers are a General Design Pattern

Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won’t cover every existing smart pointer. Many libraries have their own smart pointers, and you can even write your own. We’ll cover the most common smart pointers in the standard library:

- Box<T> for allocating values on the heap
- Rc<T>, a reference counting type that enables multiple ownership
- Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time.

In addition, we’ll cover the interior mutability pattern where an immutable type exposes an API for mutating an interior value. We’ll also discuss reference cycles: how they can leak memory and how to prevent them.
