// jkcoxson

use colored::*;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Get arguments
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        print_usage();
        return;
    }
    println!("{}", "Remote to target".green());
    println!("{}", "Target to remote".blue());

    // Create TCP server
    let listener = TcpListener::bind(args[1].clone()).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("New connection");
        let target = args[2].clone();
        tokio::spawn(async move {
            let (mut socket_read, mut socket_write) = tokio::io::split(socket);
            // Put socket in an arc so it can be cloned

            let destination = tokio::net::TcpStream::connect(target).await.unwrap();
            let (mut destination_read, mut destination_write) = tokio::io::split(destination);

            // Client listener loop
            tokio::spawn(async move {
                loop {
                    // Read from the socket
                    let mut buf = vec![0; 2097151];
                    let n = socket_read.read(&mut buf).await.unwrap();
                    if n == 0 {
                        println!("{}", "Client disconnected".red());
                        break; // Connection closed
                    }
                    let buf = &buf[..n];

                    print!("\n");
                    for i in buf {
                        print!("{}, ", format!("{:X}", i).to_string().green());
                    }
                    println!("");
                    destination_write.write_all(buf).await.unwrap();
                }
            });
            // Server listener loop
            loop {
                // Read from the socket
                let mut buf = vec![0; 2097151];
                let n = destination_read.read(&mut buf).await.unwrap();
                if n == 0 {
                    println!("{}", "Server disconnected".red());
                    break; // Connection closed
                }
                let buf = &buf[..n];

                print!("\n");
                for i in buf {
                    print!("{}, ", format!("{:X}", i).to_string().blue());
                }
                println!("");
                socket_write.write_all(buf).await.unwrap();
            }
        });
    }
}

fn print_usage() {
    println!("Usage: ./packet_passthrough <listen:port> <target:port>");
}
