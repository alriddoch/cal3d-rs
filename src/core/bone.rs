use std::rc::Rc;

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
    // CalVector        m_translationAbsolute;
    // CalQuaternion    m_rotationAbsolute;
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
}
