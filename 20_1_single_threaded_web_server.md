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

- The reason is that the `TcpStream` instance keeps track of what data it returns to us internally.  
- It might read more data than we asked for and save that data for the next time we ask for data.  
- __It therefore needs to be `mut` because its internal state might change; usually, we think of "reading" as not needing mutation, but in this case we need the `mut` keyword.__

Next, we need to actually read from the stream, `stream.read(&mut buffer).unwrap();`. We do this in two steps:

1. we declare a buffer on the stack to hold the data that is read in, at 512 bytes in size. 
  - If we wanted to handle requests of an arbitrary size, buffer management would need to be more complicated. 
  - We pass the buffer to `stream.read`, which will read bytes from the `TcpStream` and put them in the buffer.  

2. Second, we convert the bytes in the buffer to a string and print that string. 
  - The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String` from it. 
  - The "lossy" part of the name indicates the behavior of this function when it sees an invalid UTF-8 sequence: it will replace the invalid sequence with �, the U+FFFD REPLACEMENT CHARACTER. You might see replacement characters for characters in the buffer that aren’t filled by request data.

try it out:

```
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
```

Depending on your browser, you might get slightly different output.  

```
# this is with cURL

Request: GET / HTTP/1.1
Host: localhost:7878
User-Agent: curl/7.64.1
Accept: */*
```

Now that we’re printing the request data, we can see why we get multiple connections from one browser request by looking at the path after Request: `GET`. If the repeated connections are all requesting `/`, we know the browser is trying to fetch `/` repeatedly because it’s not getting a response from our program.  

Let’s break down this request data to understand what the browser is asking of our program:

## A Closer Look at an HTTP Request

`HTTP` is a text-based protocol, and a request takes this format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is the request line that holds information about what the client is requesting. The first part of the request line indicates the method being used, such as `GET` or `POST`, which describes how the client is making this request. Our client used a `GET` request.  

The next part of the request line is `/`, which indicates the Uniform Resource Identifier (URI) the client is requesting:

- a URI is almost, but not quite, the same as a Uniform Resource Locator (URL).

The last part is the HTTP version the client uses, and then the request line ends in a CRLF sequence. (CRLF stands for carriage return and line feed, which are terms from the typewriter days!) The CRLF sequence can also be written as \r\n, where \r is a carriage return and \n is a line feed. The CRLF sequence separates the request line from the rest of the request data. Note that when the CRLF is printed, we see a new line start rather than \r\n.

Looking at the request line data we received from running our program so far, we see that `GET` is the method, `/` is the request URI, and HTTP/1.1 is the version.

After the request line, the remaining lines starting from `Host`: onward are headers. `GET` requests have no body.

Try making a request from a different browser or asking for a different address, such as `127.0.0.1:7878/test`, to see how the request data changes.

## Writing a Response

### Implement sending data in response to a client request

Responses have the following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

1. The first line

- a status line that contains the HTTP version used in the response, 
- a numeric status code that summarizes the result of the request, 
- and a reason phrase that provides a text description of the status code.  

After the CRLF sequence are any headers, another CRLF sequence, and the body of the response.

Here is an example response:

- that uses HTTP version 1.1, 
- has a status code of 200, 
- an OK reason phrase, no headers, and no body:

```
HTTP/1.1 200 OK\r\n\r\n
```

The status code `200` is the standard success response. The text is a tiny successful HTTP response.  

Let’s write this to the stream as our response to a successful request from the `handle_connection` function:

```rust
// Writing a tiny successful HTTP response to the stream
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // convert string data to bytes, then send as payload in stream.write
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap();
}
```

- `response` holds the success message's data. 
- then call `as_bytes` on `response` to convert the string data to bytes. The write method on stream takes a `&[u8]` and sends those bytes directly down the connection.  

Because the `write` operation could fail, we use `unwrap` or add error handling here. Finally, `flush` will wait and prevent the program from continuing until all the bytes are written to the connection; TcpStream contains an internal buffer to minimize calls to the underlying operating system.  

With these changes, let’s run our code and make a request. We’re no longer printing any data to the terminal, so we won’t see any output other than the output from Cargo. When you load `127.0.0.1:7878` in a web browser, you should get a blank page instead of an error  

## Returning Real HTML

create a new file, `hello.html` in the root of your project dir, not in the `src` dir. You can input any HTML you want

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

- to return this from the server when a request is received, modify `handle_connection` to read the `HTML` file, add it to the response as a body, and send it.

```rust
// sending the contents of hello.html as the body of the response
use std::fs;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

- bring the standard library’s filesystem module into scope
- use `format!` to add the file’s contents as the body of the success response
- run this code with cargo run and load `127.0.0.1:7878` in your browser; and see your HTML rendered

The code is ignoring the request data in buffer and just sending back the contents of the HTML file unconditionally. That means if you try requesting `127.0.0.1:7878/something-else` in your browser, you’ll still get back this same HTML response. Our server is very limited and is not what most web servers do.  

Let's customize our responses depending on the request and only send back the HTML file for a well-formed request to `/`.

## Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the client requested.  
Let’s add functionality to check that the browser is requesting `/` before returning the HTML file and return an error if the browser requests anything else.  
For this we need to modify `handle_connection` so it checks the content of the request received against what we know a request for `/` looks like and adds `if` and `else` blocks to treat requests differently

```rust
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // create an empty buffer
    let mut buffer = [0; 512];
    // read the stream data into the buffer
    stream.read(&mut buffer).unwrap();

    let get_request_prefix = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get_request_prefix) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // some other request
    }
}
```

1. hardcode the data corresponding to the `/` request into `get_request_prefix`
  - Because we’re reading raw bytes into the buffer, we transform `get_request_prefix` into a byte string by adding the `b""` byte string syntax at the start of the content data
  - Then we check whether buffer starts with the bytes in `get_request_prefix`
  - If so, it means we’ve received a well-formed request to `/`, which is the success case we’ll handle in the if block that returns the contents of our HTML file.

If buffer does not start with the bytes in `get_request_prefix`, it means we’ve received some other request. We’ll add code to the else block in a moment to respond to all other requests.

Request `127.0.0.1:7878` and get the `HTML` in `hello.html`. If you request `127.0.0.1:7878/something-else`, you’ll get a connection error.  

Now let’s add the code to the `else` block to return a response with the status code 404, which signals that the content for the request was not found. We’ll also return some HTML for a page to render in the browser indicating the response to the end user.

```rust
// --snip--

// Responding with status code 404 and an error page if anything other than / was requested
} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Here, our response has status code 404 and the reason phrase `NOT FOUND`. We’re still not returning headers, and the body of the response will be the HTML in the file 404.html. You’ll need to create a `404.html` file next to `hello.html` for the error page

```rust
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

Now, requesting `127.0.0.1:7878` should return the contents of `hello.html`, and any other request, like `127.0.0.1:7878/foo`, should return the error HTML from `404.html`.

## A Touch of Refactoring

There is a lot of repetition in `if` and `else`: 

- they’re both reading files and writing the contents of the files to the stream. The only differences are the status line and the filename. Let’s make the code more concise by pulling out those differences into separate if and else lines that will assign the values of the status line and the filename to variables

```rust
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--
    // 'if let' assignment here into a tuple
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Now the if and else blocks only return the appropriate values for the status line and filename in a tuple:

- then destructure to assign these two values to `status_line` and `filename` using a pattern in the let statement

The previously duplicated code is now outside the if and else blocks and uses the `status_line` and `filename` variables. This makes it easier to see the difference between the two cases, and it means we have only one place to update the code if we want to change how the file reading and response writing work

- a simple web server in ~40 lines of Rust that responds to one request with a page of content and responds to all other requests with a 404 response.

This server runs in a single thread, meaning it can only serve one request at a time. Let’s examine how that can be a problem by simulating some slow requests. Then we’ll fix it so our server can handle multiple requests at once.

