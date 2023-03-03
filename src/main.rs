#![allow(dead_code)]

use crate::controller::Controller;
use std::collections::HashMap;

mod controller;
mod node;

fn main() {
    let mut ctrl = Controller{ nodes: HashMap::new() };

    for _i in 1..=3{
        ctrl.add_new_node();

    }


    ctrl.get_node(1).send_msg(String::from("hello"), String::from("127.0.0.1:20002"));
    ctrl.get_node(1).send_msg(String::from("hello"), String::from("127.0.0.1:20003"));

    ctrl.get_node(3).send_msg(String::from("hi"), String::from("127.0.0.1:20001"));

    loop {

    }
}
