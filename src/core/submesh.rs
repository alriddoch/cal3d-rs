use crate::{CalIndex, CalVector};

use super::submorphtarget::CalCoreSubMorphTarget;

pub enum CalMorphTargetType {
    CalMorphTargetTypeNull = 0,
    CalMorphTargetTypeAdditive,
    CalMorphTargetTypeClamped,
    CalMorphTargetTypeAverage,
    CalMorphTargetTypeExclusive,
}

#[derive(Clone, Default)]
pub struct TextureCoordinate {
    pub u: f32,
    pub v: f32,
}

impl TextureCoordinate {
    pub fn new() -> Self {
        TextureCoordinate { u: 0.0, v: 0.0 }
    }

    pub fn from_values(u: f32, v: f32) -> Self {
        TextureCoordinate { u, v }
    }
}

#[derive(Clone)]
pub struct TangentSpace {
    tangent: CalVector<f32>,
    crossFactor: f32, // To get the binormal, use ((N x T) * crossFactor)
}

impl Default for TangentSpace {
    fn default() -> Self {
        TangentSpace {
            tangent: CalVector::new(0.0, 0.0, 0.0),
            crossFactor: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct Influence {
    boneId: i32,
    weight: f32,
}

#[derive(Clone, Default)]
pub struct PhysicalProperty {
    weight: f32,
}

impl PhysicalProperty {
    pub fn new(weight: f32) -> Self {
        PhysicalProperty { weight }
    }
}

#[derive(Clone)]
pub struct Vertex {
    position: CalVector<f32>,
    normal: CalVector<f32>,
    vectorInfluence: Vec<Influence>,
    collapseId: i32,
    faceCollapseCount: i32,
    vertexColor: CalVector<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: CalVector::new(0.0, 0.0, 0.0),
            normal: CalVector::new(0.0, 0.0, 0.0),
            vectorInfluence: Vec::new(),
            collapseId: 0,
            faceCollapseCount: 0,
            vertexColor: CalVector::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Default)]
pub struct Face {
    vertexId: [CalIndex; 3],
}

impl Face {
    pub fn new(vertexId: [CalIndex; 3]) -> Self {
        Face { vertexId }
    }
}

/// The core submesh Spring.
#[derive(Clone, Default)]
pub struct Spring {
    vertexId: [i32; 2],
    springCoefficient: f32,
    idleLength: f32,
}

impl Spring {
    pub fn from_values(vertexId: [i32; 2], springCoefficient: f32, idleLength: f32) -> Self {
        Spring {
            vertexId,
            springCoefficient,
            idleLength,
        }
    }
}

#[derive(Default)]
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

impl CalCoreSubmesh {
    pub fn new(
        m_coreMaterialThreadId: i32,
        m_lodCount: i32,
        vertexCount: i32,
        textureCoordinateCount: i32,
        faceCount: i32,
        springCount: i32,
    ) -> Self {
        CalCoreSubmesh {
            m_vectorVertex: vec![Vertex::default(); vertexCount as usize],
            m_vectorTangentsEnabled: vec![false; textureCoordinateCount as usize],
            m_vectorvectorTangentSpace: vec![
                vec![TangentSpace::default(); vertexCount as usize];
                textureCoordinateCount as usize
            ],
            m_vectorvectorTextureCoordinate: vec![
                vec![
                    TextureCoordinate::default();
                    vertexCount as usize
                ];
                textureCoordinateCount as usize
            ],
            m_vectorFace: vec![Face::default(); faceCount as usize],
            m_vectorSpring: vec![Spring::default(); springCount as usize],
            m_vectorPhysicalProperty: vec![PhysicalProperty::default(); vertexCount as usize],

            m_coreMaterialThreadId,
            m_lodCount,
            ..Default::default()
        }
    }
}
