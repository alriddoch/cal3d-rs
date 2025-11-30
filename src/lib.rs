#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub mod animation;
pub mod bone;
pub mod core;
pub mod mesh;
pub mod mixer;
pub mod model;
pub mod morphtargetmixer;
pub mod physique;
pub mod renderer;
pub mod skeleton;
pub mod springsystem;
pub mod submesh;
pub mod vector;

pub use animation::{CalAnimation, CalAnimationAction, CalAnimationCycle};
pub use bone::CalBone;
pub use mesh::CalMesh;
pub use mixer::{CalAbstractMixer, CalMixer, CalMixerTrait};
pub use model::CalModel;
pub use morphtargetmixer::CalMorphTargetMixer;
pub use physique::CalPhysique;
pub use renderer::CalRenderer;
pub use skeleton::CalSkeleton;
pub use springsystem::CalSpringSystem;
pub use submesh::CalSubmesh;

pub use cgmath::Quaternion as CalQuaternion;
pub use cgmath::Vector3 as CalVector;

pub use i32 as CalIndex;

type UserData = Box<dyn std::any::Any>;

pub fn footle() {
    println!("Hello, world!");
}
