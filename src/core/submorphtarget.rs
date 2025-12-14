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
    m_morphTargetID: usize,
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

    pub fn setMorphID(&mut self, i: usize) {
        self.m_morphTargetID = i;
    }

    //120
    /*****************************************************************************/
    /** Sets a specified blend vertex.
     *
     * This function sets a specified blend vertex in the core sub morph target instance.
     *
     * @param vertexId  The ID of the vertex.
     * @param vertex The vertex that should be set.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn setBlendVertex(&mut self, blendVertexId: usize, blendVertex: &BlendVertex) -> bool {
        if blendVertexId >= self.m_vectorBlendVertex.len() {
            return false;
        }

        /*  if( self.m_vectorBlendVertex[blendVertexId] == NULL ) {
          self.m_vectorBlendVertex[blendVertexId] = new BlendVertex();
        }*/
        self.m_vectorBlendVertex[blendVertexId].position = blendVertex.position;
        self.m_vectorBlendVertex[blendVertexId].normal = blendVertex.normal;
        self.m_vectorBlendVertex[blendVertexId]
            .textureCoords
            .clear();
        self.m_vectorBlendVertex[blendVertexId]
            .textureCoords
            .reserve(blendVertex.textureCoords.len());
        for tcI in 0..blendVertex.textureCoords.len() {
            self.m_vectorBlendVertex[blendVertexId]
                .textureCoords
                .push(blendVertex.textureCoords[tcI].clone());
        }

        return true;
    }
}
