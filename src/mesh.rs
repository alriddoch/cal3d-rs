use crate::CalSubmesh;
use crate::core::CalCoreMesh;
use std::{cell::RefCell, rc::Rc};

pub struct CalMesh {
    m_pCoreMesh: Rc<RefCell<CalCoreMesh>>,
    m_vectorSubmesh: Vec<CalSubmesh>,
}

impl CalMesh {
    pub fn new(core_mesh: Rc<RefCell<CalCoreMesh>>) -> Self {
        CalMesh {
            m_pCoreMesh: core_mesh,
            m_vectorSubmesh: Vec::new(),
        }
    }

    pub fn getCoreMesh(&self) -> &Rc<RefCell<CalCoreMesh>> {
        &self.m_pCoreMesh
    }
}
