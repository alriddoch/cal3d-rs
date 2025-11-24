use super::skeleton::CalCoreSkeleton;
use crate::vector::bounding::BoundingBox;
use crate::{CalQuaternion, CalVector};
use std::cell::RefCell;
use std::rc::Rc;
use std::{default, ops::Mul};

pub enum CalLightType {
    LIGHT_TYPE_NONE,
    LIGHT_TYPE_OMNI,
    LIGHT_TYPE_DIRECTIONAL,
    LIGHT_TYPE_TARGET,
    LIGHT_TYPE_AMBIENT,
}

pub struct CalCoreBone {
    m_strName: String,
    m_pCoreSkeleton: Rc<RefCell<CalCoreSkeleton>>,
    m_parentId: i32,
    m_listChildId: Vec<i32>,
    m_translation: CalVector<f32>,
    m_rotation: CalQuaternion<f32>,
    m_translationAbsolute: CalVector<f32>,
    m_rotationAbsolute: CalQuaternion<f32>,
    m_translationBoneSpace: CalVector<f32>,
    m_rotationBoneSpace: CalQuaternion<f32>,
    // Cal::UserData    m_userData;
    m_boundingBox: BoundingBox,
    m_boundingPosition: [CalVector<f32>; 6],
    m_boundingBoxPrecomputed: bool,
}

impl CalCoreBone {
    pub fn new(
        m_strName: String,
        m_pCoreSkeleton: Rc<RefCell<CalCoreSkeleton>>,
        m_parentId: i32,
        m_listChildId: Vec<i32>,
        m_translation: CalVector<f32>,
        m_rotation: CalQuaternion<f32>,
        m_translationBoneSpace: CalVector<f32>,
        m_rotationBoneSpace: CalQuaternion<f32>,
    ) -> Self {
        CalCoreBone {
            m_strName,
            m_pCoreSkeleton,
            m_parentId,
            m_listChildId: Vec::<i32>::new(),
            m_translation,
            m_rotation,
            m_translationAbsolute: CalVector::<f32>::new(0.0, 0.0, 0.0),
            m_rotationAbsolute: CalQuaternion::<f32>::new(1.0, 0.0, 0.0, 0.0),
            m_translationBoneSpace,
            m_rotationBoneSpace,
            m_boundingBox: BoundingBox::default(),
            m_boundingPosition: [CalVector::<f32>::new(0.0, 0.0, 0.0); 6],
            m_boundingBoxPrecomputed: false,
        }
    }

    pub fn getName(&self) -> &str {
        &self.m_strName
    }

    pub fn getParentId(&self) -> i32 {
        self.m_parentId
    }

    pub fn addChildId(&mut self, childId: i32) {
        self.m_listChildId.push(childId);
    }

    pub fn getTranslation(&self) -> &CalVector<f32> {
        &self.m_translation
    }

    pub fn getRotation(&self) -> &CalQuaternion<f32> {
        &self.m_rotation
    }

    pub fn getRotationAbsolute(&self) -> &CalQuaternion<f32> {
        &self.m_rotationAbsolute
    }
    /**return updated absoltue transaltion.**/
    pub fn getTranslationAbsolute(&self) -> &CalVector<f32> {
        &self.m_translationAbsolute
    }

    pub fn isBoundingBoxPrecomputed(&self) -> bool {
        self.m_boundingBoxPrecomputed
    }

    pub fn setBoundingBoxPrecomputed(&mut self, inComputed: bool) {
        self.m_boundingBoxPrecomputed = inComputed;
    }

    //65
    /*****************************************************************************/
    /** Calculates the current state.
     *
     * This function calculates the current state (absolute translation and
     * rotation) of the core bone instance and all its children.
     *****************************************************************************/

    pub fn calculateState(&mut self) {
        if self.m_parentId == -1 {
            // no parent, this means absolute state == relative state
            self.m_translationAbsolute = self.m_translation;
            self.m_rotationAbsolute = self.m_rotation;
        } else {
            // get the parent bone
            let pParent = self.m_pCoreSkeleton.borrow().getCoreBone(self.m_parentId);

            if pParent.is_some() {
                // transform relative state with the absolute state of the parent
                // self.m_translationAbsolute = self.m_translation;
                // self.m_translationAbsolute *= pParent.getRotationAbsolute();
                // self.m_translationAbsolute += pParent.getTranslationAbsolute();
                let parent = pParent.as_ref().unwrap().borrow();

                self.m_translationAbsolute = parent.getRotationAbsolute().mul(self.m_translation);
                self.m_translationAbsolute =
                    self.m_translationAbsolute + parent.getTranslationAbsolute();

                self.m_rotationAbsolute = self.m_rotation.mul(parent.getRotationAbsolute());
            } else {
                eprintln!(
                    "Invalid parent bone Id {} in calculateState",
                    self.m_parentId
                );
            }
        }

        // calculate all child bones

        for iteratorChildId in self.m_listChildId.iter() {
            let bone = self.m_pCoreSkeleton.borrow().getCoreBone(*iteratorChildId);
            if bone.is_some() {
                bone.unwrap().borrow_mut().calculateState();
            } else {
                eprintln!(
                    "Invalid child bone Id {} in calculateState",
                    self.m_parentId
                );
            }
        }
    }

    // 160 cpp
    pub fn initBoundingBox(&mut self) {
        use cgmath::Rotation;
        let rot = self.m_rotationBoneSpace.clone();

        rot.invert();

        let dir = CalVector::<f32>::new(1.0, 0.0, 0.0);
        let dir = rot.mul(dir);
        self.m_boundingBox.plane[0].setNormal(&dir);

        let dir = CalVector::<f32>::new(-1.0, 0.0, 0.0);
        let dir = rot.mul(dir);
        self.m_boundingBox.plane[1].setNormal(&dir);

        let dir = CalVector::<f32>::new(0.0, 1.0, 0.0);
        let dir = rot.mul(dir);
        self.m_boundingBox.plane[2].setNormal(&dir);

        let dir = CalVector::<f32>::new(0.0, -1.0, 0.0);
        let dir = rot.mul(dir);
        self.m_boundingBox.plane[3].setNormal(&dir);

        let dir = CalVector::<f32>::new(0.0, 0.0, 1.0);
        let dir = rot.mul(dir);
        self.m_boundingBox.plane[4].setNormal(&dir);

        let dir = CalVector::<f32>::new(0.0, 0.0, -1.0);
        let dir = rot.mul(dir);
        self.m_boundingBox.plane[5].setNormal(&dir);
    }

    /*****************************************************************************/
    /** Updates the bounding box to include the given position.
     *
     * This function Updates the bounding box of the core bone instance to include
     * a given position.
     *
     * @param position The position to be included in the bounding box
     * @return True if the bounding box was changed by this call, false otherwise
     *****************************************************************************/
    pub fn updateBoundingBox(&mut self, position: &CalVector<f32>) -> bool {
        let mut bBoundsComputed = false;

        for planeId in 0..6 {
            if self.m_boundingBox.plane[planeId].Eval(position) < 0.0 {
                self.m_boundingBox.plane[planeId].SetPosition(position);
                self.m_boundingPosition[planeId] = position.clone();
                bBoundsComputed = true;
            }
        }

        return bBoundsComputed;
    }
}
