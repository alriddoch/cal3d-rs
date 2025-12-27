use crate::CalModel;
use std::{cell::RefCell, rc::Rc};

pub struct MorphAnimData {
    isManual: bool,
    animatedMorphID: i32,

    weight: f32,
    looping: bool,

    playTime: f32,
    currentWeight: f32,
    fadeIn: f32,
    fadeInTime: f32,
    fadeOut: f32,
    fadeOutTime: f32,
}

pub struct CalMorphTargetMixer {
    mAnimList: Vec<MorphAnimData>,
    m_pModel: Rc<RefCell<CalModel>>,
}

impl CalMorphTargetMixer {
    pub fn new(model: Rc<RefCell<CalModel>>) -> Self {
        Self {
            mAnimList: Vec::new(),
            m_pModel: model,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        todo!();
    }
}
