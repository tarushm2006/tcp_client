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
                Ok(socket) => {
                    println!("Connected suucesfully to the server");
                    println!("You can type in the terminal to send data to the server");
                    println!("To escape, enter ^q");
                    let (mut reader, mut writer) = socket.into_split();

                    tokio::spawn(async move {
                        loop {
                            let mut buffer = vec![0; 1024];
                            reader.read_buf(&mut buffer).await.unwrap();
                            let res = String::from_utf8_lossy(&buffer);
                            println!(">> {}", res);
                        }
                    });

                    loop {
                        let mut message = String::new();
                        io::stdin().read_line(&mut message).unwrap();
                        if message.trim() == "^q" {
                            break;
                        }
                        writer.write_all(&message.as_bytes()).await.unwrap();
                    }
                }
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
