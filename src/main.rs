#![allow(dead_code)]

use std::thread;
use std::time::Duration;
use crate::controller::Controller;
use crate::messages::*;

mod controller;
mod node;
mod messages;
mod request;

fn main() {
    let mut ctrl = Controller::new();

    for _i in 1..=3{
        ctrl.add_new_node();

    }

    thread::sleep(Duration::from_secs(2));

    ctrl.command(1, CTRL_MSG, String::from("hello"));

    loop {

    }
}
