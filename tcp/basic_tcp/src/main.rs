use std::net::TcpListener;

// This example just starts a TCP server listening at local address and port 
// To test: 
//          you can use 'nc 0.0.0.0 8888' to connect a client to this TCP socket
//          after establishing the connection and printing the address the program will finish.
//          we can also use the Browser to test to the server

fn main() {

    let listener : TcpListener = TcpListener::bind("0.0.0.0:8888").expect("Couldn't bind the address/port");

    match listener.accept() {
        Ok((_socket, addr)) => println!("Received Connection from: {addr:?}"),
        Err(e) => println!("Couldn't get client: {e:?}"),
    }

}
