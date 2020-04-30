use std::io;
use std::net::TcpListener;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!: \n{:?}", stream);
    }

    Ok(())
}
