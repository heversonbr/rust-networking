use std::net::UdpSocket;
use std::time::Duration;

fn main() -> std::io::Result<()> {

    // Binds to localhost with port 0, which tells the OS to assign any available port
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.connect("127.0.0.1:5555")?;
    // For UDP, "connect" doesn't establish a connection like TCP but rather:
    // - Sets a default destination for send() calls
    // - Restricts incoming packets to only those from the specified address

    // Sends sequentially numbered messages
    for i in 1..=5 {
        let msg = format!("Message #{}", i);
        socket.send(msg.as_bytes())?;
        // uses send() instead of send_to() because the socket is 'connected'

        let mut buf = [0u8; 1024];
        //let (amt, *) = socket.recv*from(&mut buf)?;
        // amt captures the number of bytes received
        let amt = socket.recv(&mut buf)?;

        println!("Server replied: {}", String::from_utf8_lossy(&buf[..amt]));

        // Delay Between Messages: Pauses for 500ms between messages
        std::thread::sleep(Duration::from_millis(500));
    }


    // Final notes:
    // Simple and straightforward implementation
    // Tests basic functionality of the server
    // Easy to modify for different testing scenarios
    // Single-threaded client, so doesn't test concurrent handling

    Ok(())
}