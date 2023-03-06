#![allow(dead_code)]

use crate::controller::Controller;
use crate::messages::*;

mod controller;
mod node;
mod messages;

fn main() {
    let mut ctrl = Controller::new();

    for _i in 1..=3{
        ctrl.add_new_node();

    }

    ctrl.command(1, CTRL_MSG, String::from("hello"));

    loop {

    }
}
