use std::cell::RefCell;
use std::rc::Rc;

use super::submesh::CalCoreSubmesh;

pub struct CalCoreMesh {
    m_vectorCoreSubmesh: Vec<Rc<RefCell<CalCoreSubmesh>>>,
    m_name: String,
}
