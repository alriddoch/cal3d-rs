use crate::CalBone;
use crate::core::CalCoreSkeleton;
use std::{cell::RefCell, rc::Rc};

pub struct CalSkeleton {
    m_pCoreSkeleton: Rc<RefCell<CalCoreSkeleton>>,
    m_vectorBone: Vec<Rc<RefCell<CalBone>>>,
    m_isBoundingBoxesComputed: bool,
}

impl CalSkeleton {
    pub fn new(core_skeleton: Rc<RefCell<CalCoreSkeleton>>) -> Self {
        let skeleton = core_skeleton.borrow();
        let vectorCoreBone = skeleton.getVectorCoreBone();

        let mut vectorBone = Vec::with_capacity(vectorCoreBone.len());

        for core_bone in vectorCoreBone.iter() {
            let bone = Rc::new(RefCell::new(CalBone::new(core_bone)));
            vectorBone.push(bone);
        }
        drop(skeleton);
        Self {
            m_pCoreSkeleton: core_skeleton.clone(),
            m_vectorBone: vectorBone,
            m_isBoundingBoxesComputed: false,
        }
    }

    pub fn set_bone_skeleton(&self, skeleton: &Rc<RefCell<CalSkeleton>>) {
        for bone in self.m_vectorBone.iter() {
            bone.borrow_mut().setSkeleton(skeleton);
        }
    }

    pub fn getVectorBone(&self) -> &Vec<Rc<RefCell<CalBone>>> {
        &self.m_vectorBone
    }

    // 77 cpp
    /*****************************************************************************/
    /** Calculates the state of the skeleton instance.
     *
     * This function calculates the state of the skeleton instance by recursively
     * calculating the states of its bones.
     *****************************************************************************/
    pub fn calculateState(&mut self) {
        // calculate all bone states of the skeleton
        let listRootCoreBoneId = self.m_pCoreSkeleton.borrow().getVectorRootCoreBoneId();

        for iteratorRootBoneId in listRootCoreBoneId.iter() {
            self.m_vectorBone[*iteratorRootBoneId]
                .borrow_mut()
                .calculateState();
        }
        self.m_isBoundingBoxesComputed = false;
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
            iteratorBone.borrow_mut().clearState();
        }
        self.m_isBoundingBoxesComputed = false;
    }

    // 115
    /*****************************************************************************/
    /** Locks the state of the skeleton instance.
     *
     * This function locks the state of the skeleton instance by recursively
     * locking the states of its bones.
     *****************************************************************************/

    pub fn lockState(&mut self) {
        // lock all bone states of the skeleton

        for iteratorBone in self.m_vectorBone.iter() {
            iteratorBone.borrow_mut().lockState();
        }
    }
}
