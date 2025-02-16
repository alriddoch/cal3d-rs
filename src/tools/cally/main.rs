mod demo;
mod menu;
mod model;

use demo::*;
use model::Model;

fn main() {
    cal3d::footle();

    let mut demo = Demo::new();

    demo.OnCreate().expect("Demo create failed");

    demo.OnInit().expect("Demo init failed");

    demo.Loop();
}
