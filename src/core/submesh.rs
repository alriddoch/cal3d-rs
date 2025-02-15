use crate::{CalIndex, CalVector};

use super::submorphtarget::CalCoreSubMorphTarget;

pub enum CalMorphTargetType {
    CalMorphTargetTypeNull = 0,
    CalMorphTargetTypeAdditive,
    CalMorphTargetTypeClamped,
    CalMorphTargetTypeAverage,
    CalMorphTargetTypeExclusive,
}

pub struct TextureCoordinate {
    u: f32,
    v: f32,
}

pub struct TangentSpace {
    tangent: CalVector<f32>,
    crossFactor: f32, // To get the binormal, use ((N x T) * crossFactor)
}

pub struct Influence {
    boneId: i32,
    weight: f32,
}

pub struct PhysicalProperty {
    weight: f32,
}

pub struct Vertex {
    position: CalVector<f32>,
    normal: CalVector<f32>,
    vectorInfluence: Vec<Influence>,
    collapseId: i32,
    faceCollapseCount: i32,
    vertexColor: CalVector<f32>,
}

pub struct Face {
    vertexId: [CalIndex; 3],
}

/// The core submesh Spring.
pub struct Spring {
    vertexId: [i32; 2],
    springCoefficient: f32,
    idleLength: f32,
}

pub struct CalCoreSubmesh {
    m_vectorVertex: Vec<Vertex>,
    m_vectorTangentsEnabled: Vec<bool>,
    m_vectorvectorTangentSpace: Vec<Vec<TangentSpace>>,
    m_vectorvectorTextureCoordinate: Vec<Vec<TextureCoordinate>>,
    m_vectorPhysicalProperty: Vec<PhysicalProperty>,
    m_vectorFace: Vec<Face>,
    m_vectorSpring: Vec<Spring>,
    m_vectorCoreSubMorphTarget: Vec<CalCoreSubMorphTarget>,
    m_coreMaterialThreadId: i32,
    m_lodCount: i32,
    m_vectorSubMorphTargetGroupIndex: Vec<u32>,
    m_hasNonWhiteVertexColors: bool,
}
