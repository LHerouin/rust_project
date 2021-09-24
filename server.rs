use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

fn handle_client(mut stream: i32, mut stream_list: Arc<Mutex<HashMap<i32,TcpStream>>>) {
    
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    //let mut i = 0;
   /*while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
           /* if i==0{
                println!("{}", String::from_utf8_lossy(&data[..]));
                i=i+1;
            }*/
            
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}*/
    
}


fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    let mut streamHash:HashMap<i32,TcpStream> = HashMap::new(); 
    let mut replies_stream = Arc::new(Mutex::new(streamHash));
    let mut i = 0;
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                println!("New connection: {}", &stream.peer_addr().unwrap());
                let mut rs = replies_stream.lock().unwrap();
                (*rs).insert(
                    i,
                    stream,
                );
                i = i+1;
                
                let replies_stream_clone = replies_stream.clone();
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(i, replies_stream_clone)
                });
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