use crate::CalModel;
use std::{cell::RefCell, rc::Rc};

pub struct CalRenderer {
    m_pModel: Rc<RefCell<CalModel>>,
}
