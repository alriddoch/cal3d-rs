mod demo;
mod graphics;
mod menu;
mod model;
mod tick;

use demo::*;
use model::Model;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    cal3d::footle();

    let demo = Rc::new(RefCell::new(Demo::new().unwrap()));

    let demo_ref = demo.clone();

    demo.borrow_mut().OnCreate().expect("Demo create failed");

    demo.borrow_mut()
        .OnInit(demo_ref)
        .expect("Demo init failed");

    demo.borrow_mut().Loop();
}
