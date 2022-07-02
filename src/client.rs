use std::io::{prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::{str, thread, time};

pub fn start_client(address: String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address.clone())?;
    println!("TCP client connected to server {}\n", address);

    // send 10 times
    for i in 0..10 {
        let msg = format!("Hello from client {}", i);
        stream
            .write(msg.as_bytes())
            .expect("Write into stream failed");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Read from stream failed");

        // print replyed message
        println!("{}",str::from_utf8(&buffer).expect("Could not write buffer as string"));

        // sleep for 1 seconds
        thread::sleep(time::Duration::from_secs(1 as u64));
    }
    Ok(())
}
