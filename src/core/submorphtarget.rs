use std::cell::RefCell;
use std::rc::Rc;

use crate::CalVector;

use super::submesh::{CalCoreSubmesh, CalMorphTargetType, TextureCoordinate};

#[derive(Clone)]
pub struct BlendVertex {
    pub position: CalVector<f32>,
    pub normal: CalVector<f32>,
    pub textureCoords: Vec<TextureCoordinate>,
}

impl BlendVertex {
    pub fn new(textureCoordinateCount: usize) -> Self {
        let mut textureCoords = Vec::new();
        textureCoords.reserve(textureCoordinateCount);
        BlendVertex {
            position: CalVector::<f32>::new(0.0, 0.0, 0.0),
            normal: CalVector::<f32>::new(0.0, 0.0, 0.0),
            textureCoords,
        }
    }
}

impl Default for BlendVertex {
    fn default() -> Self {
        BlendVertex {
            position: CalVector::<f32>::new(0.0, 0.0, 0.0),
            normal: CalVector::<f32>::new(0.0, 0.0, 0.0),
            textureCoords: Vec::new(),
        }
    }
}

pub struct CalCoreSubMorphTarget {
    m_name: String,
    m_vectorBlendVertex: Vec<BlendVertex>,
    m_coreSubmesh: Rc<RefCell<CalCoreSubmesh>>,
    m_morphTargetID: u32,
    m_morphTargetType: CalMorphTargetType,
}

impl CalCoreSubMorphTarget {
    pub fn new(
        m_coreSubmesh: Rc<RefCell<CalCoreSubmesh>>,
        vertex_count: usize,
        m_name: String,
    ) -> Self {
        CalCoreSubMorphTarget {
            m_name,
            m_vectorBlendVertex: vec![BlendVertex::default(); vertex_count],
            m_coreSubmesh,
            m_morphTargetID: 0,
            m_morphTargetType: CalMorphTargetType::CalMorphTargetTypeAdditive,
        }
    }
}
