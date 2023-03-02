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
    socket: UdpSocket
}

impl Node{
    pub fn new(id: usize, address: String, port:usize) -> Node {
        // Create the socket address for this node
        let bind_address = format!("{}:{}", address, port.to_string());

        // Create the node struct with the given parameters and a new UDPsocket
        let node = Node{
            id, address, port,
            socket: UdpSocket::bind(bind_address)
            .expect("Couldn't bind")
        };

        // Clone the socket so that it can be moved into the other thread
        let new_socket = node.socket.try_clone().expect("Unable to clone socket");

        // Create a new thread responsible for waiting for incoming UDP messages
        thread::Builder::new().name("NodeListener".to_string()).spawn(move || {
            loop {
                // This buffer stores the incoming datagram
                let mut buffer = [0; 1024];

                // This makes sure that something only happens if the message is correctly received
                match new_socket.recv_from(&mut buffer) {
                    Ok((num_bytes, src_addr)) => {
                        let send_buffer = &mut buffer[..num_bytes];

                        println!("Received: {} from {}", str::from_utf8(send_buffer).unwrap(),
                            src_addr.to_string());
                    }
                    _ => {}
                }
            }
        }).expect("Thread failed");

        node // Return the node struct
    }

    pub fn send_msg(&self, msg:String, dest:String){
        // Connect to the destination address and send the message
        self.socket.connect(dest).expect("Couldn't connect");
        self.socket.send(msg.as_bytes()).expect("Unable to send");
    }
}