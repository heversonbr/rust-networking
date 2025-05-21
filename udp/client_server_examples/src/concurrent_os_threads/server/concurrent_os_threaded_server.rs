use std::net::UdpSocket;
use std::thread;

fn main() -> std::io::Result<()> {

    // Socket Creation and Binding:
    let addr_socket = "127.0.0.1:5555";
    // Bind the server socket to listen on port 8080, 
    // and propagates any error ('?') that might occur during binding
    let socket = UdpSocket::bind(addr_socket).expect("Failed creating a UDP socket from the given address.");
    println!("Server listening on {} ", socket.local_addr().unwrap());

    loop {  // an infinite loop, allowing the server to continuously process incoming

        // Buffer Declaration:
        // Declares a buffer to hold the incoming data
        // in this case, we create a buffer of 1024 bytes initialized with zeros to store incoming data
        let mut buf = [0u8; 1024];  //1024 bytes 

        // Receiving Data
        // Blocks until a message is received
        let (amount, source) = socket.recv_from(&mut buf)?;
        // 'amount' stores how many bytes were received
        // 'source' stores the sender's address. The message data is stored in 'buf'

        // Socket Cloning:
        // Creates a clone of the socket so it can be moved into a new thread
        let socket_clone = socket.try_clone()?;
        // Reminder: resources can't be shared between threads without proper handling.
        //           Cloning is the simplest approach when you need the socket to be shared in multiple threads
        //           Cloning the socket allows it to be moved into the new thread

        let data = buf[..amount].to_vec();
        // Ownership for the Thread: The buffer (buf) is owned by the main thread
        //                           By creating a vector with just the relevant data, 
        //                           we can move ownership of just that data into the new thread
        // Correct Data Size: the buffer is 1024 bytes, but the actual message might be smaller , 
        //                    buf[..amount] takes a slice of just the bytes that were received
        //                    'to_vec()' creates a new vector containing only those bytes
        // Lifetime Management: the thread might outlive the original buffer. 
        //                      creating a vector ensures the data exists as long as the thread needs it

        // Spawn a thread to handle the message
        thread::spawn(move || {
            let message = String::from_utf8_lossy(&data);
            println!("Received from {}: {}", source, message);
            
            // Optionally send a reply back
            let reply = format!("Echo: {}", message);
            let _ = socket_clone.send_to(reply.as_bytes(), source);
        });

        // Final consideration:
        // Creating a new vector for each message adds some overhead (memory allocation and copying). 
        // For high-performance applications, you might consider alternative designs that reduce this overhead,
        // such as: 
        // - Thread Pools: Use a fixed thread pool instead of spawning threads per message
        // - Buffer Pooling: Reuse buffers instead of allocating new vectors
        // - Process In-place: When possible, process data without copying, for instance use mio for Non-Blocking I/O
        // - *Async I/O*: Consider async runtimes like Tokio for high-concurrency scenarios
        // - Share Resources: Use Arc to share sockets instead of cloning for each message, for instance Lock-Free Ring Buffer with MPMC

    }

}
