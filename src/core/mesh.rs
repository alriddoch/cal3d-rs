use std::cell::RefCell;
use std::rc::Rc;

use super::submesh::CalCoreSubmesh;

pub struct CalCoreMesh {
    m_vectorCoreSubmesh: Vec<Rc<RefCell<CalCoreSubmesh>>>,
    m_name: String,
}

impl CalCoreMesh {
    pub fn new(m_vectorCoreSubmesh: Vec<Rc<RefCell<CalCoreSubmesh>>>) -> Self {
        CalCoreMesh {
            m_vectorCoreSubmesh,
            m_name: String::from(""),
        }
    }

    pub fn getCoreSubmeshes(&self) -> &Vec<Rc<RefCell<CalCoreSubmesh>>> {
        &self.m_vectorCoreSubmesh
    }
}
