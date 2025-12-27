use crate::CalModel;
use std::{cell::RefCell, rc::Rc};

pub struct CalRenderer {
    m_pModel: Rc<RefCell<CalModel>>,
}

impl CalRenderer {
    pub fn new(model: Rc<RefCell<CalModel>>) -> Self {
        Self { m_pModel: model }
    }
}
