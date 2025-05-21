use std::net::{UdpSocket};


fn main() -> Result<(), Box<dyn std::error::Error>>  {

    // For the client side of this second example we can use the exact same client from the previous example.
    // the server will be in a loop, which means we can send as many messages as we wish from this client. 
    
    // Bind to any free port on localhost for the client ('0' lets the OS pick the port)
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("Client bound to {}", socket.local_addr()?);

    println!("Sending message to target IP (server) : 127.0.0.1:4444");
    // Option 1: send_to – No connect needed
    //¯ocket.send_to(b"Message with no previous connection, from example 2!", "127.0.0.1:4444")?;

    // Option 2: send – Requires connect
    // note in this example that, we dont actually need to define the target IP address before, 
    // we can create it from a string as shown below. 
    socket.connect("127.0.0.1:4444")?;
    socket.send(b"<<First>> message: with previous connection, from example 2")?;
    socket.send(b"<<Second>> message: with previous connection, from example 2")?;
    socket.send(b"<<Third>> message: with previous connection, from example 2")?;

    Ok(())
}

// Other observations:
// 
// - Looping: Needed to keep listening for multiple messages.
// - Blocking: All standard socket operations block the current thread by default.
// - Async/Threaded server: Optional for advanced performance and responsiveness.

