mod server;
mod client;

fn main() {
    match server::start_server(String::from("127.0.0.1:8000")) {
        Err(e) => {
            println!("Create TCP server failed: {}", e);
        }
        Ok(_) => {
            println!("TCP server closed");
        }
    }
}
