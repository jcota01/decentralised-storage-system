use std::net::UdpSocket;
use std::thread;
use std::str;

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
    successor:String,
    predecessor:String

}

impl Node{
    pub fn new(id: usize, address: String, port:usize) {
        // Create the socket address for this node
        let bind_address = format!("{}:{}", address, port.to_string());

        // Create the node struct with the given parameters and a new UDPsocket
        let node = Node{
            id, address, port,
            socket: UdpSocket::bind(bind_address)
            .expect("Couldn't bind"),
            successor: String::new(),
            predecessor: String::new(),
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
        let node_id = self.id;

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

                        // Print out message
                        println!("{} received: {} from {}", node_id, str::from_utf8(send_buffer).unwrap(),
                            src_addr.to_string());

                        // If message isn't ACK
                        /*if str::from_utf8(send_buffer).unwrap() != "ACK"{
                            // Respond with ACK
                            new_socket.send_to(String::from("ACK").as_bytes(), src_addr.to_string()).expect(&*format!("{}", node_id));
                        }*/
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