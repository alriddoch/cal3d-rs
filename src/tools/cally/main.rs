#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod demo;
mod graphics;
mod menu;
mod model;
mod models;
mod tick;

use demo::*;

fn main() {
    cal3d::footle();

    let mut demo = Demo::new().unwrap();

    demo.OnCreate().expect("Demo create failed");

    demo.OnInit().expect("Demo init failed");

    demo.Loop();
}
