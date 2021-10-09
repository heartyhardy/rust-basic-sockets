use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main(){
  match TcpStream::connect("localhost:3500"){
    Ok(mut stream) => {
      println!("Connected to PORT: {}", 3500);

      let message = b"Hello from Client!";

      stream.write(message).unwrap();
      println!("Sent a message to server... Awaiting Response");

      let mut data = [0_u8; 18];

      match stream.read_exact(&mut data){
        Ok(_) => {
          if &data == message{
            println!("Response is valid and correct!");
          }else{
            let response = from_utf8(&data).unwrap();
            println!("Unrecognized format of response: {}", response);
          }
        },
        Err(e) => {
          println!("Failed to receive data from server! {:?}", e);
        }
      }
    },
    Err(e) =>{
      println!("Failed to connect to PORT: {}", 3500);
    }
  }
  println!("Closing Client!...");
}

