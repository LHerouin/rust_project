use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

use std::net::{IpAddr, Ipv4Addr};


pub enum Mode {
    Reconnect,
    KeepAlive,
}

pub struct Client {
    msg: [u8; 13],
    port: i32,
    address_ip: String
}
impl Client {
    pub fn new(msg:[u8; 13], port: i32, address_ip: String) -> Self {
        Client {
            msg: msg,
            port: port,
            address_ip: address_ip
        }
    }
}

fn main() {

    /*loop{
        let mut i 
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        println!("Your choice is : {}", input_text)
    }*/

    let msg = b"Hello server!";
    let client = Client::new(*msg, 3333, "localhost".to_string());

    let tcp_stream = format!("{}:{}", client.address_ip, client.port);

print!("{}", tcp_stream);
   match TcpStream::connect(tcp_stream) {
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_client_port() {
        let msg = b"Hello server!";
        let client = Client::new(*msg, 3333, "localhost".to_string());
        assert_eq!(client.port, 3333);
    }

    #[test]
    #[should_panic]
    fn test_client_address_ip() {
        let msg = b"Hello server!";
        let client = Client::new(*msg, 3333, "localhost".to_string());
        assert_eq!(client.address_ip, "3333");
    }

    #[test]
    fn test_client_tcp_connect() {
        let msg = b"Hello server!";
        let client = Client::new(*msg, 3333, "127.0.0.1".to_string());
        let tcp_stream = format!("{}:{}", client.address_ip, client.port);

        let stream = TcpStream::connect(tcp_stream).expect("Aucune connexion n’a pu être établie car l’ordinateur cible l’a expressément refusée.");

        assert_eq!(stream.local_addr().unwrap().ip(),
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    }
}
