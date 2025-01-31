use std::rc::Rc;
use std::{default, ops::Mul};

use crate::{CalQuaternion, CalVector};

use super::skeleton::CalCoreSkeleton;

pub enum CalLightType {
    LIGHT_TYPE_NONE,
    LIGHT_TYPE_OMNI,
    LIGHT_TYPE_DIRECTIONAL,
    LIGHT_TYPE_TARGET,
    LIGHT_TYPE_AMBIENT,
}

pub struct CalCoreBone {
    m_strName: String,
    m_pCoreSkeleton: Rc<CalCoreSkeleton>,
    m_parentId: i32,
    m_listChildId: Vec<i32>,
    m_translation: CalVector<f32>,
    m_rotation: CalQuaternion<f32>,
    m_translationAbsolute: CalVector<f32>,
    m_rotationAbsolute: CalQuaternion<f32>,
    m_translationBoneSpace: CalVector<f32>,
    m_rotationBoneSpace: CalQuaternion<f32>,
    // Cal::UserData    m_userData;

    // CalBoundingBox   m_boundingBox;
    // CalVector        m_boundingPosition[6];
    // bool             m_boundingBoxPrecomputed;
}

impl CalCoreBone {
    pub fn new(
        m_strName: String,
        m_pCoreSkeleton: Rc<CalCoreSkeleton>,
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

    pub fn getRotationAbsolute(&self) -> &CalQuaternion<f32> {
        &self.m_rotationAbsolute
    }
    /**return updated absoltue transaltion.**/
    pub fn getTranslationAbsolute(&self) -> &CalVector<f32> {
        &self.m_translationAbsolute
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
            let pParent = self.m_pCoreSkeleton.getCoreBone(self.m_parentId);

            // transform relative state with the absolute state of the parent
            // self.m_translationAbsolute = self.m_translation;
            // self.m_translationAbsolute *= pParent.getRotationAbsolute();
            // self.m_translationAbsolute += pParent.getTranslationAbsolute();

            self.m_translationAbsolute = pParent
                .borrow()
                .getRotationAbsolute()
                .mul(self.m_translation);
            self.m_translationAbsolute =
                self.m_translationAbsolute + pParent.borrow().getTranslationAbsolute();

            self.m_rotationAbsolute = self.m_rotation.mul(pParent.borrow().getRotationAbsolute());
        }

        // calculate all child bones

        for iteratorChildId in self.m_listChildId.iter() {
            self.m_pCoreSkeleton
                .getCoreBone(*iteratorChildId)
                .borrow_mut()
                .calculateState();
        }
    }
}
