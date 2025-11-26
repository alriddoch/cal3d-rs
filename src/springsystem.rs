use crate::CalModel;
use std::{cell::RefCell, rc::Rc};

pub struct CalSpringSystem {
    m_pModel: Rc<RefCell<CalModel>>,
}

impl CalSpringSystem {
    pub fn update(&mut self, delta_time: f32) {
        todo!();
    }
}
