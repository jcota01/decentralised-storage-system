#![allow(dead_code)]

use crate::controller::Controller;
use std::{thread, time::Duration};

mod controller;
mod node;

fn main() {
    let mut ctrl = Controller{ nodes: Vec::new() };

    for _i in 1..=3{
        ctrl.add_new_node();

    }

    thread::sleep(Duration::from_secs(1));

    ctrl.nodes[0].send_msg(String::from("hello"), String::from("127.0.0.1:20002"));
    ctrl.nodes[0].send_msg(String::from("hello"), String::from("127.0.0.1:20003"));

    ctrl.nodes[2].send_msg(String::from("hi"), String::from("127.0.0.1:20001"));

    loop {

    }
}
