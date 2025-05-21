use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};



fn main() -> Result<(), Box<dyn std::error::Error>>  {

    // For the client side, we will create the target IPv4 address and bind it to the local end
    // as we did it for the server, by defining a local address and port for the client side or 
    // simply assinging an address and letting the operating system to choose a free port. 
    
    // Bind to any free port on localhost for the client (0 lets the OS pick the port)
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("Client bound to {}", socket.local_addr()?);

    // just like receiving has 'recv_from' and 'recv', sending in UDP also has two key methods in Rust’s standard library:
    // 'send_to(buf, target_addr)' : For Unconnected Sockets) , and 
    // 'send(buf)' :  After .connect()
    // and which one you use depends on whether the socket is connected or not

    // Before using one of the options lets create the target IPv4 address: the server address , to send the message to.
    let server_ip = Ipv4Addr::new(127, 0, 0, 1);
    let server_port = 3333;
    let server_addr = SocketAddrV4::new(server_ip, server_port);

    println!("Created target IP (server) : {} ", server_addr);
    // Option 1: send_to – No connect needed - uses send_to() method
    socket.send_to(b"Message with no previous connection!", server_addr)?;

    // Option 2: send – Requires connect - uses send() method
    // note in this example that, we dont actually need to define the target IP address before, 
    // we can create it from a string as shown below. 
    // socket.connect("127.0.0.1:8080")?;
    // socket.send(b"ping with previous connection")?;

    Ok(())

}


// Takeaway:
//
// - UdpSocket::bind(...)  : Binds a socket to a specific local address/port (server or client).
//
// - SocketAddrV4          : Represents a combination of an IPv4 address and a port. IPv4-specific address + port
//
// - SocketAddrV6          : Represents a combination of an IPv6 specific address + port + extras
//
// - SocketAddr            : Generic enum for both IPv4 and IPv6
//
// - IpAddr                : IP only (v4 or v6), Ex: 127.0.0.1 or ::1,  General IP handling
//
// - Ipv4Addr::new(...)    : Constructs an IPv4 address from four u8 values.
//
// - recv_from(&mut buf) -> Result<(usize, SocketAddr)>  
//                         : Receives a datagram from the socket, returns number of bytes received and sender address.
//                           Waits for a UDP packet (blocking by default). Receives a message from any client; 
//
// - recv(&mut buf) -> Result<usize> 
//                         : Receives a datagram but does NOT give you the sender address. Only works for a known peer; simpler, but only one peer.
//                           Useful only if the socket is connected to a remote peer via .connect(). Also blocking by default.
//
// - send_to()             : Sends a datagram to the specified destination address. No need to connect().
//                           Can send to different addresses every time. Perfect for a client sending to multiple servers, or unconnected communication.
//
// -  send(buf)            : Sends data to a predefined peer (set by connect()). Cannot change the target unless you connect() again.
//                           Simpler to use (send() instead of send_to()). Slightly faster, because the OS doesn’t have to look up the destination each time.
//
// - pub fn connect(&self, addr: ToSocketAddrs) -> Result<()> 
//                         : Connects this UDP socket to a remote address, allowing the send and recv 
//                           syscalls to be used to send data and also applies filters to only receive data from the specified address.
//                           Doesn’t create a persistent connection like TCP.  Instead, it sets a default peer address.  
//                           After that, .send() and .recv() behave like TCP I/O — but still using UDP under the hood.
//                           This improves: i) Performance (no need to specify destination every time). ii) Simplicity (fewer arguments to pass).
//
// Other observations:
// 
// - Looping: Needed to keep listening for multiple messages.
// - Blocking: All standard socket operations block the current thread by default.
// - Async/Threaded server: Optional for advanced performance and responsiveness.

