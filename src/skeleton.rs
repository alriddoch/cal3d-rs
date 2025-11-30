use crate::CalBone;
use crate::core::CalCoreSkeleton;
use std::{cell::RefCell, rc::Rc};

pub struct CalSkeleton {
    m_pCoreSkeleton: Rc<RefCell<CalCoreSkeleton>>,
    m_vectorBone: Vec<CalBone>,
    m_isBoundingBoxesComputed: bool,
}

impl CalSkeleton {
    pub fn new(core_skeleton: Rc<RefCell<CalCoreSkeleton>>) -> Self {
        Self {
            m_pCoreSkeleton: core_skeleton,
            m_vectorBone: Vec::new(),
            m_isBoundingBoxesComputed: false,
        }
    }
}
