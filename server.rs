use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

fn handle_client(mut stream: i32, mut stream_list: Arc<Mutex<HashMap<i32,TcpStream>>>) {
    
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    let mut my_stream = {
        let mut ls = stream_list.lock().unwrap();
        (*ls).get(&stream).unwrap().try_clone().unwrap()
    };
    //let mut i = 0;
    while match my_stream.read(&mut data) {
        Ok(size) => {
            for (key, mut val) in stream_list.lock().unwrap().iter(){
                println!("key: {} val: {:?}", key, val);
                if *key != stream {
                    val.write(&data[0..size]).unwrap();
                    //println!("{}", String::from_utf8_lossy(&data[..]));
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
                
                let replies_stream_clone = replies_stream.clone();
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(i, replies_stream_clone)
                });
                i = i+1;
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
