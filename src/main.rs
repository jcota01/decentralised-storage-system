#![allow(dead_code)]

use crate::controller::Controller;

mod controller;
mod node;

fn main() {
    let mut ctrl = Controller{
        nodes: Vec::new()
    };

    for _i in 1..=5{
        ctrl.add_new_node();
    }

    println!("{:?}", ctrl)


}
