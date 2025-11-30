use crate::CalSkeleton;
use crate::core::CalCoreBone;
use crate::vector::bounding::BoundingBox;
use crate::{CalQuaternion, CalVector};
use cgmath::{Matrix3, SquareMatrix};
use std::{cell::RefCell, rc::Rc};

pub struct CalBone {
    m_pCoreBone: Rc<CalCoreBone>,
    m_pSkeleton: Rc<CalSkeleton>,
    m_accumulatedWeight: f32,
    m_accumulatedWeightAbsolute: f32,
    m_accumulatedReplacementAttenuation: f32,
    m_firstBlendScale: f32,
    m_meshScaleAbsolute: CalVector<f32>, // w.r.t. absolute coord system in 3dsMax (Z up), not local coord of bone.
    m_translation: CalVector<f32>,
    m_rotation: CalQuaternion<f32>,
    m_translationAbsolute: CalVector<f32>,
    m_rotationAbsolute: CalQuaternion<f32>,
    m_translationBoneSpace: CalVector<f32>,
    m_rotationBoneSpace: CalQuaternion<f32>,
    m_transformMatrix: Matrix3<f32>,
    m_boundingBox: BoundingBox,
}
