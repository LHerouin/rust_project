use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {

    /*loop{
        let mut i 
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        println!("Your choice is : {}", input_text)
    }*/
    


   match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
           

            let msg = b"Hello server!";

            stream.write(msg).unwrap();
            println!("Sent Hello server, awaiting reply...");

            let mut data = [0 as u8; 13]; // using 6 byte buffer
            let mut validate = false;
            while match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                        println!("{:?}", from_utf8(&data).unwrap());
                        validate = true;
                        
                    } else if validate {
                        let text = from_utf8(&data).unwrap();
                        println!("{}", text);
                    }
                    true
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    false
                }
            }{}
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}