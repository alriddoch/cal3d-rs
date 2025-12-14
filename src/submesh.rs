use crate::core::CalCoreSubmesh;

pub struct CalSubmesh {
    m_pCoreSubmesh: CalCoreSubmesh,
    // std::vector<float>                      m_vectorMorphTargetWeight;
    // std::vector<float>                      m_vectorAccumulatedWeight;
    // std::vector<float>                      m_vectorReplacementAttenuation;
    // std::vector<CalVector>                  m_vectorVertex;
    // std::vector<CalVector>                  m_vectorNormal;
    // std::vector<std::vector<TangentSpace> > m_vectorvectorTangentSpace;
    // std::vector<Face>                       m_vectorFace;
    // std::vector<PhysicalProperty>           m_vectorPhysicalProperty;
    // std::vector<int>                        m_vectorSubMorphTargetGroupAttenuator;
    // std::vector<float>                      m_vectorSubMorphTargetGroupAttenuation;
    // int                                     m_vertexCount;
    // int                                     m_faceCount;
    m_coreMaterialId: i32,
    // bool                                    m_bInternalData;
}

impl CalSubmesh {
    pub fn getCoreSubmesh(&self) -> &CalCoreSubmesh {
        &self.m_pCoreSubmesh
    }

    pub fn setCoreMaterialId(&mut self, coreMaterialId: i32) {
        self.m_coreMaterialId = coreMaterialId;
    }
}
