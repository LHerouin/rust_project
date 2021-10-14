use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{IpAddr};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub struct Server {
    id_stream: i32,
    server_name: String,
    port: i32,
    address_ip: String
}

impl Server {
    pub fn new(id_stream: i32, server_name:String, port: i32, address_ip: String) -> Self {
        Server {
            id_stream: id_stream,
            server_name: server_name,
            port: port,
            address_ip: address_ip
        }
    }
}

fn handle_client(stream: i32, stream_list: Arc<Mutex<HashMap<i32,TcpStream>>>) {
    
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    let mut my_stream = {
        let ls = stream_list.lock().unwrap();
        (*ls).get(&stream).unwrap().try_clone().unwrap()
    };
    //let mut i = 0;
    while match my_stream.read(&mut data) {
        Ok(size) => {
            for (key, mut val) in stream_list.lock().unwrap().iter(){
                println!("key: {} val: {:?}", key, val);
                if *key != stream {
                    val.write(&data[0..size]).unwrap();
                }
            } 
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", my_stream.peer_addr().unwrap());
            my_stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
    
}


fn main() {
    let server = Server::new(0, "My Server".to_string(), 3333, "0.0.0.0".to_string());

    let tcp_stream = format!("{}:{}", server.address_ip, server.port);


    let listener = TcpListener::bind(tcp_stream).unwrap();
    let streamHash:HashMap<i32,TcpStream> = HashMap::new(); 
    let replies_stream = Arc::new(Mutex::new(streamHash));
    let mut id = server.id_stream;

    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                println!("New connection: {}", &stream.peer_addr().unwrap());
                let mut rs = replies_stream.lock().unwrap();
                (*rs).insert(
                    id,
                    stream,
                );
                
                let replies_stream_clone = replies_stream.clone();
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(id, replies_stream_clone)
                });
                id = id+1;
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}


#[cfg(test)] 
mod tests {
    use super::*;


    #[test]
    fn test_server_name() {
        
        let server = Server::new(0, "My Server".to_string(), 3333, "0.0.0.0".to_string());
       assert_eq!(server.server_name,"My Server");
    }        

    #[test]
    fn test_server_tcp_listener() {
        
        let server = Server::new(0, "My Server".to_string(), 3333, "0.0.0.0".to_string());
        let tcp_stream = format!("{}:{}", server.address_ip, server.port);
        let stream = TcpListener::bind(tcp_stream).unwrap();
        assert_eq!(stream.local_addr().unwrap(),
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3333)));
    }


}
