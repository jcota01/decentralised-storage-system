use crate::node::Node;

#[derive(Debug)]
pub struct Controller {
    pub nodes: Vec<Node>
}

impl Controller {

    // Create a new node and add it to the vector
    pub fn add_new_node(&mut self){
        self.nodes.push(Node::new(
            self.nodes.len() + 1,
            "127.0.0.1".to_string(),
            20001 + self.nodes.len(),
        ));
    }
}