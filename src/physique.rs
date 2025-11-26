use crate::CalModel;
use std::{cell::RefCell, rc::Rc};

pub struct CalPhysique {
    m_pModel: Rc<RefCell<CalModel>>,
}

impl CalPhysique {
    pub fn update(&mut self) {
        todo!();
    }
}
