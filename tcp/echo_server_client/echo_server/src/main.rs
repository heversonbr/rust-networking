
#[warn(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

// TcpListener: https://doc.rust-lang.org/stable/std/net/struct.TcpListener.html#method.accept

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{

    println!("Incoming connection from {}", stream.peer_addr()?);
    // declare a buffer to read the data from the client
    let mut buffer = [0; 512];

    loop {
        // reads the bytes from the stream and stores the data in the buffer,
        // it returns the amount of bytes read from the stream
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");

        // if we want to print these bytes locally, we can do the following
        // it will convert the data in the buffer into utf8 encoded string
        let str_msg_from_client = String::from_utf8_lossy(&buffer[..]);
        println!("Msg received from client: {}" , str_msg_from_client);

        // here we just check if there's nothing else to read from the stream.
        // if the amount of bytes read is equal zero, then there's notthing else to read, then returns Ok 
        if bytes_read == 0 { return Ok(())}

        // as a part of the echo logic, it sends back to the sender (writes to the open stream) the bytes read before
        stream.write("Hello I'm an Echo Server, I've just received this msg from you: ".as_bytes()).expect("Failed while writing the server reponse");
        stream.write(&buffer[..bytes_read]).expect("Failed while writing the server reponse");
    }
}
 

fn main() {
    
    // struct TcpListener: A TCP socket server, listening for connections.
    // We create TcpListener by binding it to a socket address.
    // After binding the TcpListener, it will listen for incoming TCP connections. 
    // When a connection is established, it can be accepted in 2 different ways:
    // 1) by calling 'accept' function which accepts a new incoming connection from this listener
    //    This function will block the calling thread until a new TCP connection is established.
    // 2) by calling 'incoming' function: this function returns an iterator called 'Incoming' over the connections being received on this listener.
    //    we iterate over this iterator, which is equivalent to calling TcpListener::accept in a loop
    
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Couldn't bind the address/port");
    println!("Server listening at 0.0.0.0:8888");
    // in this example we will use the incoming function:
    // Returns an iterator over the connections being received on this listener.
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("FAILED: {}", e) } 
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }

    // you can use 'nc 0.0.0.0 8888' to connect a client to this TCP socket

}
