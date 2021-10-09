use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn client_handler(mut stream: TcpStream){
    let mut data = [0_u8; 50];
    while match stream.read(&mut data){
        Ok(size) => {            
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("Error Occured! Shutting down connection: {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap_or_default();
            false
        }
    }{}
}


fn main() {
    let listener =TcpListener::bind("0.0.0.0:3500").unwrap();
    
    println!("Server Running on PORT: {}", 3500);

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection accepted: {}", stream.peer_addr().unwrap());

                thread::spawn(move || {
                    client_handler(stream);
                });
            },
            Err(e) => {
                println!("Connection Error! {:?}", e);
            }
        }
    }
    drop(listener);
}
