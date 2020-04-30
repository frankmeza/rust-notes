# Building a Single-Threaded Web Server

- start by getting a single-threaded web server working
  
## a quick overview of the protocols involved in building web servers

The details of these protocols are beyond the scope of this book, but a brief overview will give you the information you need.

The two main protocols involved in web servers

- Hypertext Transfer Protocol (`HTTP`), and
- the Transmission Control Protocol (`TCP`)
  
Both protocols are request-response protocols, meaning a client initiates requests and a server listens to the requests and provides a response to the client.  

The contents of those requests and responses are defined by the protocols.

`TCP`  

- the lower-level protocol that describes how info gets from one server to another 
- but doesn’t specify what that information is.  

`HTTP`

- builds on top of `TCP` by defining the contents of the requests and responses
- it’s technically possible to use `HTTP` with other protocols, but in the vast majority of cases, `HTTP` sends its data over `TCP`. 

We’ll work with the raw bytes of `TCP` and `HTTP` requests and responses:  

## Start a new Cargo project

```
$ cargo new tcp_client
    Created binary (application) `tcp_client` project
$ cd tcp_client
```

### Using TcpListener

- we can listen for TCP connections at the address `127.0.0.1:7878`,
  - the section before the colon is an IP address representing your computer (this is the same on every computer and doesn’t represent the authors’ computer specifically), and 
  - 7878 is the port
- We’ve chosen this port for two reasons: `HTTP` is normally accepted on this port, and 7878 is rust typed on a telephone.

The `bind` function works like a `*::new()` function, returning a new `TcpListener` instance.  

The reason the function is called `bind` is that in networking, connecting to a port to listen to is known as "binding to a port."  

The `bind` function

- returns a `Result<T, E>`, which indicates that binding might fail. 
- For example, connecting to port `80` requires admin privileges (nonadmins can listen only on ports higher than 1024), so if we tried to connect to port `80` without being an admin, binding would fail
- As another example, binding fails if we ran two instances of our program listening to the same port

Because we’re writing a basic server just for learning purposes, we won’t worry about handling these kinds of errors; instead, we use unwrap to stop the program if errors happen.

-----

The `incoming` method on `TcpListener`

- returns an iterator that gives us a sequence of streams (more specifically, streams of type `TcpStream`)
- A single stream represents an open connection between the client and the server
- A `connection` is the name for the full request and response process in which
  - a client connects to the server,
  - the server generates a response, 
  - and the server closes the connection. 
  
As such, `TcpStream` will read from itself
- to see what the client sent and then allow us to write our response to the stream. 

Overall, this `for` loop will process each connection in turn and produce a series of streams for us to handle.  

The reason we might receive errors from the `incoming` method when a client connects to the server is that we’re not actually iterating over connections. Instead, we’re iterating over connection attempts.  

The connection might not be successful for a number of reasons, many of them operating system specific. For example, many operating systems have a limit to the number of simultaneous open connections they can support; new connection attempts beyond that number will produce an error until some of the open connections are closed.

#### Invoke cargo run in the terminal and then load 127.0.0.1:7878 in a web browser

The browser should show an error message like "Connection reset" because the server isn’t currently sending back any data. But when you look at your terminal, you should see several messages that were printed when the browser connected to the server!

```
Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

- multiple messages printed for one browser request occur when the browser is making a request for the page as well as a request for other resources, ex favicon

It could also be that the browser is trying to connect to the server multiple times because the server isn’t responding with any data.  

- When stream goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation. Browsers sometimes deal with closed connections by retrying, because the problem might be temporary.  

The important factor is that we’ve successfully gotten a handle to a TCP connection!

#### stop the program by pressing ctrl-c when you’re done running a particular version of the code. Then restart cargo run after you’ve made each set of code changes to make sure you’re running the newest code.

### Reading the Request

```rust
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

We bring std::io::prelude into scope to get access to certain traits that let us read from and write to the stream.  

In the `for` loop in the `main` function:  

- instead of printing a message that says we made a connection,
- we now call the new `handle_connection` function and pass the stream to it.  

In the `handle_connection` function, we’ve made the stream parameter mutable, `let mut buffer = [0; 512];`.  
The reason is that the `TcpStream` instance keeps track of what data it returns to us internally. It might read more data than we asked for and save that data for the next time we ask for data. It therefore needs to be `mut` because its internal state might change; usually, we think of "reading" as not needing mutation, but in this case we need the `mut` keyword.

Next, we need to actually read from the stream, `stream.read(&mut buffer).unwrap();`. We do this in two steps:

1. we declare a buffer on the stack to hold the data that is read in, at 512 bytes in size. 
  - If we wanted to handle requests of an arbitrary size, buffer management would need to be more complicated. 
  - We pass the buffer to `stream.read`, which will read bytes from the `TcpStream` and put them in the buffer.  

2. Second, we convert the bytes in the buffer to a string and print that string. 
  - The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String` from it. 
  - The "lossy" part of the name indicates the behavior of this function when it sees an invalid UTF-8 sequence: it will replace the invalid sequence with �, the U+FFFD REPLACEMENT CHARACTER. You might see replacement characters for characters in the buffer that aren’t filled by request data.

Let’s try this code! Start the program and make a request in a web browser again. Note that we’ll still get an error page in the browser, but our program’s output in the terminal will now look similar to this:


$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
Depending on your browser, you might get slightly different output. Now that we’re printing the request data, we can see why we get multiple connections from one browser request by looking at the path after Request: GET. If the repeated connections are all requesting /, we know the browser is trying to fetch / repeatedly because it’s not getting a response from our program.

Let’s break down this request data to understand what the browser is asking of our program.
