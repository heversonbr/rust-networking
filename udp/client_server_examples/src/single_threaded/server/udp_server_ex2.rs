use std::net::UdpSocket;

fn main()-> Result<(), Box<dyn std::error::Error>>  {

    // In this example we will use the basics we saw in example 1 and we will improve it a bit 
    // by keeping the server in a loop to receive messages from clients 

    // Create and bind an UDP socket to an IPv4 address and port (localhost:8080), simpler way
    let open_socket = UdpSocket::bind("127.0.0.1:4444").expect("Failed creating a UDP socket from the given address.");
    println!("Server listening on {} ", open_socket.local_addr().unwrap());

    // After binding an udp server the next step is to listen to remote connections, 
    // declare a buffer to hold the incoming data (max 1024 bytes in this case)
    let mut buf = [0u8; 1024];

    // Then we wait for a message. recv_from() returns the size and the sender's address.
    loop {
        let (size, sender) = open_socket.recv_from(&mut buf)?;
        let message = String::from_utf8_lossy(&buf[..size]);
        println!("Received Message from {}: {}", sender, message);
    }
    


}