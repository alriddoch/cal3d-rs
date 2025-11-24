#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub mod core;
pub mod mixer;
pub mod model;
pub mod vector;

pub use mixer::{CalAbstractMixer, CalMixer};
pub use model::CalModel;

pub use cgmath::Quaternion as CalQuaternion;
pub use cgmath::Vector3 as CalVector;

pub use i32 as CalIndex;

pub fn footle() {
    println!("Hello, world!");
}
