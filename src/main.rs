#![allow(dead_code)]

use crate::controller::Controller;

mod controller;
mod node;

fn main() {
    let mut ctrl = Controller{
        nodes: Vec::new()
    };

    for _i in 1..=2{
        ctrl.add_new_node();

    }

    ctrl.nodes[0].send_msg(String::from("hello"), String::from("127.0.0.1:20002"));
}
