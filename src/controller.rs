use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread;
use crate::node::Node;

#[derive(Debug)]
pub struct Controller {
    pub nodes: HashMap<usize, String>,
    pub socket: UdpSocket
}

impl Controller {
    pub fn new() -> Controller{
        Controller{
            nodes: HashMap::new(),
            socket: UdpSocket::bind("127.0.0.1:20000").expect("Could not bind")
        }
    }

    // Create a new node and add it to the HashMap
    pub fn add_new_node(&mut self){
        let node_id = self.nodes.len() + 1;
        let address = String::from("127.0.0.1");
        let address_clone = address.clone();

        thread::Builder::new().name(format!("{}", node_id)).spawn(move || {
            Node::new(node_id, address,  20000 + (2*node_id) - 1);
        }).expect("New node failed");

        self.nodes.insert(self.nodes.len() + 1, address_clone);
    }

    // Sends a command to a node using one of the message codes in messages.rs
    pub fn command(&self, node:usize, code:&str, msg:String){
        let address = self.nodes.get(&node).unwrap();
        let msg:String =  format!("{}{}", code, msg);
        let port:usize = 20000 + (2 * node) - 1;
        let dest = format!("{}:{}", address, port);

        self.socket.send_to(msg.as_bytes(), dest).expect("Couldn't send message");
    }
}