use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use chrono::Local;

use crate::client;

fn handle_client_stream(mut stream: TcpStream) -> Result<(), Error> {

    // use a buffer store the reading bytes
    let mut buf = [0; 1024];

    // reading bytes from client stream
    for _ in 0.. {
        let bytes_read = stream.read(&mut buf)?;

        // when read nothing, return
        if bytes_read == 0 {
            return Ok(());
        }

        // use chrono get formatted time
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");

        // print received message
        println!("[{} Client]: {}", now, String::from_utf8_lossy(&buf));

        // write back the read datum
        stream.write(String::from(format!("[{} Server]: ", now)).as_bytes())?;
        stream.write(&buf[..bytes_read])?;
        stream.write(&['\n' as u8])?;
    }
    Ok(())
}

pub fn start_server(address: String) -> std::io::Result<()> {

    // bind TCP server at address
    let listener = TcpListener::bind(address.clone())?;

    println!("TCP server started at address: {}", address);

    // store the thread join handle of every incoming client connection
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // start client for testing
    thread::spawn(move || {
        match client::start_client(address.clone()) {
            Ok(_) => {}
            Err(e) => {
                println!("Connect to server {} failed: {}", address, e);
            }
        }
    });

    // iterate all incoming client conenctions
    for stream in listener.incoming() {
        // valid and unwrap client conenction stream
        let stream = stream.expect("Valid client stream failed");
        // create a new thread to deal current client stream, use handle_client_stream function
        let handle = thread::spawn(move || {
            // handle client stream, unwrap error and foramt output
            handle_client_stream(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        // store join handle into a vec
        thread_vec.push(handle);
    }

    // join all client stream dealing thread, prevent main thread closed
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
