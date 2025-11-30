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

    pub fn getVectorBone(&self) -> &Vec<CalBone> {
        &self.m_vectorBone
    }

    // 98 cpp
    /*****************************************************************************/
    /** Clears the state of the skeleton instance.
     *
     * This function clears the state of the skeleton instance by recursively
     * clearing the states of its bones.
     *****************************************************************************/
    pub fn clearState(&mut self) {
        // clear all bone states of the skeleton
        for iteratorBone in self.m_vectorBone.iter_mut() {
            iteratorBone.clearState();
        }
        self.m_isBoundingBoxesComputed = false;
    }
}
