use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

// This is a simple echo client used to send the strings read from the std input
// to a server and get the same strings back.

fn main() {

    // TcpStream::connect => Opens a TCP connection to a remote host.
    let mut stream = TcpStream::connect("0.0.0.0:8888").expect("couldn't connect with server!");

    loop {

        // The value that will store locally the string read from the std input 
        let mut input = String::new();

        // The vector that will be used to read responses from the server
        let mut buffer: Vec<u8> = Vec::new();

        // The stdin::read_line method: Locks this handle and reads a line of input, appending it to the specified buffer.
        //    Reads a line (wait for the Enter key to be pressed before continuing)
        //    from the standard input as a string and stores the line into the variable we called 'input' 
        io::stdin().read_line(&mut input).expect("Failed while writting to server ");
        
        // we write the input string into the stream as bytes (in order to the server to read it)
        // TcpStream::write -> Write a buffer into this writer, returning how many bytes were written.
        stream.write(input.as_bytes()).expect("Fail to write the input into the stream: Fail to write to the server!");
        // at this point , the server must have read the string from the stream and sent back to the client, then
        
        // we read the stream and we store the string into the read buffer, that we call reader here
        let mut reader = BufReader::new(&stream);

        // the next 'read  until' method reads the strem into the buffer variable
        // Read all bytes into buf until the delimiter byte or EOF is reached.
        reader.read_until(b'\n', &mut buffer).expect("Couldn't rad into Buffer");

        // which is (the buffer variable) printed as a string
        // str::from_utf8  -> Converts a slice of bytes to a string slice.
        print!("{}", str::from_utf8(&buffer).expect("Couldn't read fron the buffer"));
    }

}
