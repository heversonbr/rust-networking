use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

fn main()-> Result<(), Box<dyn std::error::Error>>  {
    // Concepts: 
    // 'SocketAddr' is a Rust abstraction, of an Internet socket addresses, safe and portable across platforms.
    //  It is an enum that can be either a 'SocketAddrV4' or 'SocketAddrV6'.  
    //  It consists of an IP address ('IpAddr'), a 16-bit port number, as well as possibly some version-dependent additional information     
    // SocketAddrV4:
	// - A concrete type that represents an IPv4 socket address.
	//   It contains:  An Ipv4Addr (e.g. 127.0.0.1) and A u16 port number (e.g. 8080)
    // SocketAddrV6: 
    // - A concrete type that represents an IPv6 socket address.
    //   It contains:  An Ipv6Addr (e.g. 127.0.0.1) and A u16 port number (e.g. 8080)
    // IpAddr: 
    // It is an enum that represents either an IPv4 or an IPv6 address, without a port. General IP handling.
    //      Ipv4Addr: defined as  32-bit integers in IETF RFC 791.  They are usually represented as four octets.
    //      Ipv6Addr: defined as 128-bit integers in IETF RFC 4291. They are usually represented as eight 16-bit segments.


    // Create the target IPv4 address: 127.0.0.1:8080 (server address)
    let ip_add_v4 = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3333;
    // Create an IpAdd 
    let ip_add = IpAddr::V4(ip_add_v4);

    // Creating a internet socket addresss
    let socket_addr = SocketAddr::new(ip_add, port);
    // or, many other different ways of doing it. 
    //let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    //let server_socket_addr = SocketAddrV4::new(ip_add_v4, port);

    // Create a UDP socket bound to an IPv4 address and port (localhost:8080)
    let open_socket = UdpSocket::bind(socket_addr).expect("Failed creating a UDP socket from the given address.");
    println!("Server listening on {} ", open_socket.local_addr().unwrap());
    // we could also simply do this: let socket = UdpSocket::bind("127.0.0.1:8080") 
    // but to be more clear about everything that happens, the full process is presented above.

    // After binding an udp server the next step is to listen to remote connections, 
    // the way of doing this is by using 'recv_from' or 'recv', depending on what we want. 
    // So we can receive messages from clients using either:
    // 1) recv_from:  pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>
    //    Receives a message from any remote client, Also gives you the sender’s address (SocketAddr).
    //    Ideal for server-like behavior given the fact that it blocks (waits) until a message is received
    // 2) recv(&mut buf):  pub fn recv(&self, buf: &mut [u8]) -> Result<usize>
    //    Receives a single datagram message on the socket from the remote address to which it is connected.
	//    Can only be used after calling .connect() to a specific client.
	//    You don’t get the sender’s address. Not suitable for general servers, only for dedicated communication.

    // So here we have a server and then we will use the recv_from:
    // First we declare a buffer to hold the incoming data (max 1024 bytes in this case)
    let mut buf = [0u8; 1024];

    // Then we wait for a message. recv_from() returns the size and the sender's address.
    let (size, sender) = open_socket.recv_from(&mut buf)?;

    // In this example we know that the clients are goind to send strings 
    // So, lets convert the received bytes into a string
    let message = String::from_utf8_lossy(&buf[..size]);
    // the from_utf8_lossy(&buf[..size]) method : Converts a slice of bytes to a string, including invalid characters.
    println!("Received Message from {}: {}", sender, message);

    // At this point 

    Ok(())

}