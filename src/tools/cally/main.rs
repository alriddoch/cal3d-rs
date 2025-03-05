mod demo;
mod graphics;
mod menu;
mod model;

use demo::*;
use model::Model;

fn main() {
    cal3d::footle();

    let mut demo = Demo::new().unwrap();

    demo.OnCreate().expect("Demo create failed");

    demo.OnInit().expect("Demo init failed");

    demo.Loop();
}
