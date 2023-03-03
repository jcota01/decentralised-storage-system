use std::collections::HashMap;
use crate::node::Node;

#[derive(Debug)]
pub struct Controller {
    pub nodes: HashMap<usize, Node>
}

impl Controller {
    // Returns a Node from the HashMap
    // TODO: Handle None value
    pub fn get_node(&self, id: usize) -> &Node {
        self.nodes.get(&id).unwrap()
    }


    // Create a new node and add it to the vector
    pub fn add_new_node(&mut self){
        self.nodes.insert(
            self.nodes.len() + 1,
            Node::new(
                self.nodes.len() + 1,
                "127.0.0.1".to_string(),
                20001 + self.nodes.len(),
            )
        );
    }
}