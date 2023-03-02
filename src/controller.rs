use crate::node::Node;

#[derive(Debug)]
pub struct Controller {
    pub nodes: Vec<Node>
}

impl Controller {

    pub fn add_new_node(&mut self){
        self.nodes.push(Node {
            id: self.nodes.len() + 1,
            address: "127.0.0.1".to_string(),
            port: 20001 + self.nodes.len(),
        })
    }
}