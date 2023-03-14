use std::collections::VecDeque;
use std::net::UdpSocket;
use std::thread;
use std::str;
use std::sync::{Arc, Mutex};
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

        let mut queue:VecDeque<Box<Request>> = VecDeque::with_capacity(20);
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

        node.main_thread();
        node.listener();

    }

    // The main thread for each node
    fn main_thread(&self){
        println!("{:?}", self);
    }

    // New thread for receiving UDP messages
    fn listener(&self){
        // Clone the socket so that it can be moved into the other thread
        let new_socket = self.socket.try_clone().expect("Unable to clone socket");
        let node_id = self.id.clone();
        let listener_queue = self.queue.clone();

        // Create a new thread responsible for waiting for incoming UDP messages
        thread::Builder::new().name(format!("NodeListener {}", node_id)).spawn(move || {
            loop {
                // This buffer stores the incoming datagram
                let mut buffer = [0; 1024];

                // This makes sure that something only happens if the message is correctly received
                match new_socket.recv_from(&mut buffer) {
                    Ok((num_bytes, src_addr)) => {
                        // Use the number of bytes sent to take the message from the buffer
                        let send_buffer = &mut buffer[..num_bytes];

                        let msg = str::from_utf8(send_buffer).unwrap();
                        let msg = msg.to_string();

                        // Print out message
                        println!("{} received: {} from {}", node_id, str::from_utf8(send_buffer).unwrap(),
                            src_addr.to_string());

                        let request:Request = Request{
                            msg
                        };

                        if let Ok(mut x) = listener_queue.lock(){
                            x.push_back(
                                Box::new(request)
                            );
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