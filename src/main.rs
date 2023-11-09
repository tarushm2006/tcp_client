use std::env;
use std::{io, vec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ErrorKind},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Required arguments were not provided to the TCP client."),
        _ => {
            let stream_result = TcpStream::connect(args[1].clone()).await;
            match stream_result {
                Ok(mut stream) => loop {
                    let mut message = String::new();

                    println!("Enter your message:");
                    io::stdin().read_line(&mut message).unwrap();
                    stream.write_all(message.as_bytes()).await.unwrap();

                    let mut buffer = vec![0; message.len()];
                    stream.read_exact(&mut buffer).await.unwrap();

                    let res = String::from_utf8_lossy(&buffer);
                    println!("{}", res);
                },
                Err(error) => {
                    if error.kind() == ErrorKind::ConnectionRefused {
                        println!("The connection was refused by the host.");
                    } else {
                        println!("There is an error connecting to the host. Run the debug build to get more details.");
                    }
                }
            };
        }
    }
}
