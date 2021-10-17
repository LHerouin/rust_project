#[macro_use] extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

pub enum Mode {
    Reconnect,
    KeepAlive,
}

pub struct Client {
    msg: [u8; 24],
    port: i32,
    address_ip: String
}
impl Client {
    pub fn new(msg:[u8; 24], port: i32, address_ip: String) -> Self {
        Client {
            msg: msg,
            port: port,
            address_ip: address_ip
        }
    }
}

fn main() {
    let mc = new_magic_crypt!("magickey", 256);
    /*loop{
        let mut i 
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        println!("Your choice is : {}", input_text)
    }*/

    let base64msg = b"Twm+JdcGfbYu7vqSi/gXNA==";
    let client = Client::new(*base64msg, 3333, "localhost".to_string());

    let tcp_stream = format!("{}:{}", client.address_ip, client.port);

print!("{}", tcp_stream);
   match TcpStream::connect(tcp_stream) { // accepting a connection on a TcpListener(server)
        Ok(mut stream) => {

            stream.write(&client.msg).unwrap();// write data on stream
            println!(" Sent Hello client, awaiting reply...");

            let mut data = [0 as u8; 24]; // using 6 byte buffer (number of bytes we need to read)
            let mut validate = false;
            while match stream.read_exact(&mut data) { //read the exact number of bytes required
                Ok(_) => {
                    if &data == base64msg {
                        println!("Reply is ok!");
                        println!("{:?}", mc.decrypt_base64_to_string(from_utf8(&data).unwrap()).unwrap());// "Hello server!" (avec encodage utf8 + chiffrement)
                        validate = true;
                        
                    } else if validate {
                        let text = from_utf8(&data).unwrap();
                        println!("{}", text);
                    }
                    true
                },
                Err(e) => { //error
                    println!("Failed to receive data: {}", e);
                    false
                }
            }{}
        },
        Err(e) => { //error 
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_client_port() {
        let mc = new_magic_crypt!("magickey", 256);
        //let msg = b"Hello client!";
        let base64msg = b"Twm+JdcGfbYu7vqSi/gXNA==";
        //let base64 = mc.encrypt_str_to_base64(&msg);
        let client = Client::new(*base64msg, 3333, "localhost".to_string());
        assert_eq!(client.port, 3333);
    }

    #[test]
    #[should_panic]
    fn test_client_address_ip() {
        let mc = new_magic_crypt!("magickey", 256);
        let base64msg = b"Twm+JdcGfbYu7vqSi/gXNA==";
        let client = Client::new(*base64msg, 3333, "localhost".to_string());
        assert_eq!(client.address_ip, "3333");
    }

    #[test]
    fn test_client_tcp_connect() { // il faut lancer le server (cargo run) pour que ça fonctionne
        //let msg = b"Hello client!";
        let base64msg = b"Twm+JdcGfbYu7vqSi/gXNA==";
        let client = Client::new(*base64msg, 3333, "127.0.0.1".to_string());
        let tcp_stream = format!("{}:{}", client.address_ip, client.port);

        let stream = TcpStream::connect(tcp_stream).expect("Aucune connexion n’a pu être établie car l’ordinateur cible l’a expressément refusée.");

        assert_eq!(stream.local_addr().unwrap().ip(),
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    }
}
