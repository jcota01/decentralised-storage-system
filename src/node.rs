use std::collections::VecDeque;
use std::net::UdpSocket;
use std::thread;
use std::str;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::request::Request;

/*
This struct is used to represent each node on the system. Each node acts completely independently
from one another.
 */
#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub address: String,
    pub port: usize,
    socket: UdpSocket,
    queue: Arc<Mutex<VecDeque<Box<Request>>>>,
    successor:String,
    predecessor:String

}

impl Node{
    pub fn new(id: usize, address: String, port:usize) {
        // Create the socket address for this node
        let bind_address = format!("{}:{}", address, port.to_string());

        let queue:VecDeque<Box<Request>> = VecDeque::with_capacity(20);
        let queue = Mutex::new(queue);
        let queue = Arc::new(queue);

        // Create the node struct with the given parameters and a new UDPsocket
        let node = Node{
            id, address, port,
            socket: UdpSocket::bind(bind_address).expect("Couldn't bind"),
            successor: String::new(),
            predecessor: String::new(),
            queue
        };

        node.listener();
        node.main_thread();
    }

    // The main thread for each node
    fn main_thread(&self){
        loop {
            let mut request:Option<Box<Request>> = None;
            if let Ok(mut x) = self.queue.lock(){
                if !x.is_empty(){
                    request = x.pop_front();
                }
            };

            match request {
                None => {}
                Some(x) => {println!("{:?}", x)}
            };

            thread::sleep(Duration::from_millis(500));
        }
    }

    // New thread for receiving UDP messages
    fn listener(&self){
        // Clone variables for the new thread
        let new_socket = self.socket.try_clone().expect("Unable to clone socket");
        let node_id = self.id.clone();
        let listener_queue = self.queue.clone();


        thread::Builder::new().name(format!("NodeListener {}", node_id)).spawn(move || {
            loop {
                // This buffer stores the incoming datagram
                let mut buffer = [0; 1024];

                // This makes sure that something only happens if the message is correctly received
                match new_socket.recv_from(&mut buffer) {
                    Ok((num_bytes, src_addr)) => {
                        // Use the number of bytes sent to take the message from the buffer
                        let send_buffer = &mut buffer[..num_bytes];

                        let msg = str::from_utf8(send_buffer).unwrap().to_string();
                        let request = Request{ msg };

                        // Access the queue to push the request
                        if let Ok(mut x) = listener_queue.lock(){
                            x.push_back(Box::new(request));
                        };
                    }
                    _ => {}
                }
            }
        }).expect("Thread failed");
    }

    pub fn send_msg(&self, msg:String, dest:String){
        self.socket.send_to(msg.as_bytes(), dest).expect("Couldn't send message");
    }
}