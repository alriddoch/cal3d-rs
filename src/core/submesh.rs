use cgmath::InnerSpace;

use crate::{CalIndex, CalVector};

use super::submorphtarget::CalCoreSubMorphTarget;

#[derive(Default)]
pub enum CalMorphTargetType {
    #[default]
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

#[derive(Clone, Default)]
pub struct Influence {
    pub boneId: i32,
    pub weight: f32,
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
    pub position: CalVector<f32>,
    pub normal: CalVector<f32>,
    pub vectorInfluence: Vec<Influence>,
    pub collapseId: i32,
    pub faceCollapseCount: i32,
    pub vertexColor: CalVector<f32>,
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
    pub vertexId: [CalIndex; 3],
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
        vertexCount: usize,
        textureCoordinateCount: usize,
        faceCount: usize,
        springCount: usize,
    ) -> Self {
        CalCoreSubmesh {
            m_vectorVertex: vec![Vertex::default(); vertexCount],
            m_vectorTangentsEnabled: vec![false; textureCoordinateCount],
            m_vectorvectorTangentSpace: vec![
                vec![TangentSpace::default(); vertexCount];
                textureCoordinateCount
            ],
            m_vectorvectorTextureCoordinate: vec![
                vec![TextureCoordinate::default(); vertexCount];
                textureCoordinateCount
            ],
            m_vectorFace: vec![Face::default(); faceCount],
            m_vectorSpring: vec![Spring::default(); springCount],
            m_vectorPhysicalProperty: vec![PhysicalProperty::default(); vertexCount],

            m_coreMaterialThreadId,
            m_lodCount,
            ..Default::default()
        }
    }

    pub fn setHasNonWhiteVertexColors(&mut self, v: bool) {
        self.m_hasNonWhiteVertexColors = true;
    }

    // 96 cpp
    /*****************************************************************************/
    /** Returns the ID of the core material thread.
     *
     * This function returns the ID of the core material thread of this core
     * submesh instance.
     *
     * @return The ID of the core material thread.
     *****************************************************************************/

    pub fn getCoreMaterialThreadId(&self) -> i32 {
        self.m_coreMaterialThreadId
    }

    // 136 cpp
    /*****************************************************************************/
    /** Returns the number of springs.
     *
     * This function returns the number of springs in the core submesh instance.
     *
     * @return The number of springs.
     *****************************************************************************/
    pub fn getSpringCount(&self) -> usize {
        return self.m_vectorSpring.len();
    }

    //165
    /*****************************************************************************/
    /** UpdateTangentVector
     *
     *****************************************************************************/

    fn UpdateTangentVector(&mut self, v0: i32, v1: i32, v2: i32, mapId: usize) {
        let vvtx = self.getVectorVertex();
        let vtex = &self.m_vectorvectorTextureCoordinate[mapId];

        let v0 = v0 as usize;
        let v1 = v1 as usize;
        let v2 = v2 as usize;

        // Step 1. Compute the approximate tangent vector.
        let du1 = vtex[v1].u - vtex[v0].u;
        let dv1 = vtex[v1].v - vtex[v0].v;
        let du2 = vtex[v2].u - vtex[v0].u;
        let dv2 = vtex[v2].v - vtex[v0].v;

        let prod1 = (du1 * dv2 - dv1 * du2);
        let prod2 = (du2 * dv1 - dv2 * du1);
        if (((prod1.abs()) < 0.000001) || ((prod2.abs()) < 0.000001)) {
            return;
        }

        let x = dv2 / prod1;
        let y = dv1 / prod2;

        let vec1 = vvtx[v1].position - vvtx[v0].position;
        let vec2 = vvtx[v2].position - vvtx[v0].position;
        let mut tangent = (vec1 * x) + (vec2 * y);

        // Step 2. Orthonormalize the tangent.
        let component = CalVector::dot(tangent, vvtx[v0].normal);
        tangent -= &vvtx[v0].normal * component;
        tangent.normalize();

        // Step 3: Add the estimated tangent to the overall estimate for the vertex.

        self.m_vectorvectorTangentSpace[mapId][v0].tangent += tangent;
    }

    /*****************************************************************************/
    /** Enables (and calculates) or disables the storage of tangent spaces.
     *
     * This function enables or disables the storage of tangent space bases.
     *****************************************************************************/
    pub fn enableTangents(&mut self, mapId: usize, enabled: bool) -> bool {
        if (mapId < 0) || (mapId >= self.m_vectorTangentsEnabled.len()) {
            return false;
        }

        self.m_vectorTangentsEnabled[mapId] = enabled;

        if (!enabled) {
            self.m_vectorvectorTangentSpace[mapId].clear();
            return true;
        }

        let new_len = self.m_vectorVertex.len();
        let old_capacity = self.m_vectorvectorTangentSpace.capacity();

        if new_len > old_capacity {
            self.m_vectorvectorTangentSpace[mapId].reserve(new_len - old_capacity);
        }
        self.m_vectorvectorTangentSpace[mapId].resize(new_len, TangentSpace::default());

        for tangentId in 0..self.m_vectorvectorTangentSpace[mapId].len() {
            self.m_vectorvectorTangentSpace[mapId][tangentId].tangent =
                CalVector::new(0.0, 0.0, 0.0);
            self.m_vectorvectorTangentSpace[mapId][tangentId].crossFactor = 1.0;
        }

        for faceId in 0..self.m_vectorFace.len() {
            self.UpdateTangentVector(
                self.m_vectorFace[faceId].vertexId[0],
                self.m_vectorFace[faceId].vertexId[1],
                self.m_vectorFace[faceId].vertexId[2],
                mapId,
            );
            self.UpdateTangentVector(
                self.m_vectorFace[faceId].vertexId[1],
                self.m_vectorFace[faceId].vertexId[2],
                self.m_vectorFace[faceId].vertexId[0],
                mapId,
            );
            self.UpdateTangentVector(
                self.m_vectorFace[faceId].vertexId[2],
                self.m_vectorFace[faceId].vertexId[0],
                self.m_vectorFace[faceId].vertexId[1],
                mapId,
            );
        }

        for tangentId in 0..self.m_vectorvectorTangentSpace[mapId].len() {
            self.m_vectorvectorTangentSpace[mapId][tangentId]
                .tangent
                .normalize();
        }

        return true;
    }

    //528
    /*****************************************************************************/
    /** Sets a specified face.
     *
     * This function sets a specified face in the core submesh instance.
     *
     * @param faceId  The ID of the face.
     * @param face The face that should be set.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn setFace(&mut self, faceId: usize, face: Face) -> bool {
        if (faceId < 0) || (faceId >= self.m_vectorFace.len()) {
            return false;
        }

        self.m_vectorFace[faceId] = face;

        return true;
    }

    //591
    /*****************************************************************************/
    /** Sets a specified physical property.
     *
     * This function sets a specified physical property in the core submesh
     * instance.
     *
     * @param vertexId  The ID of the vertex.
     * @param physicalProperty The physical property that should be set.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn setPhysicalProperty(
        &mut self,
        vertexId: usize,
        physicalProperty: PhysicalProperty,
    ) -> bool {
        if (vertexId < 0) || (vertexId >= self.m_vectorPhysicalProperty.len()) {
            return false;
        }

        self.m_vectorPhysicalProperty[vertexId] = physicalProperty;

        return true;
    }

    /*****************************************************************************/
    /** Sets a specified spring.
     *
     * This function sets a specified spring in the core submesh instance.
     *
     * @param springId  The ID of the spring.
     * @param spring The spring that should be set.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn setSpring(&mut self, springId: usize, spring: Spring) -> bool {
        if (springId < 0) || (springId >= self.m_vectorSpring.len()) {
            return false;
        }

        self.m_vectorSpring[springId] = spring;

        return true;
    }

    //636
    /*****************************************************************************/
    /** Sets a specified texture coordinate.
     *
     * This function sets a specified texture coordinate in the core submesh
     * instance.
     *
     * @param vertexId  The ID of the vertex.
     * @param textureCoordinateId  The ID of the texture coordinate.
     * @param textureCoordinate The texture coordinate that should be set.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn setTextureCoordinate(
        &mut self,
        vertexId: usize,
        textureCoordinateId: usize,
        textureCoordinate: TextureCoordinate,
    ) -> bool {
        if (textureCoordinateId < 0)
            || (textureCoordinateId >= self.m_vectorvectorTextureCoordinate.len())
        {
            return false;
        }
        if (vertexId < 0)
            || (vertexId >= self.m_vectorvectorTextureCoordinate[textureCoordinateId].len())
        {
            return false;
        }

        self.m_vectorvectorTextureCoordinate[textureCoordinateId][vertexId] = textureCoordinate;

        return true;
    }

    pub fn setAllTextureCoordinates(&mut self, texture_coordinates: Vec<Vec<TextureCoordinate>>) {
        self.m_vectorvectorTextureCoordinate = texture_coordinates;
    }

    //683
    /*****************************************************************************/
    /** Adds a core sub morph target.
     *
     * This function adds a core sub morph target to the core sub mesh instance.
     *
     * @param pCoreSubMorphTarget A pointer to the core sub morph target that should be added.
     *
     * @return One of the following values:
     *         \li the assigned sub morph target \b ID of the added core sub morph target
     *         \li \b -1 if an error happened
     *****************************************************************************/

    pub fn addCoreSubMorphTarget(
        &mut self,
        mut pCoreSubMorphTarget: CalCoreSubMorphTarget,
    ) -> usize {
        // get next sub morph target id

        let subMorphTargetId = self.m_vectorCoreSubMorphTarget.len();
        pCoreSubMorphTarget.setMorphID(subMorphTargetId);
        self.m_vectorCoreSubMorphTarget.push(pCoreSubMorphTarget);

        // This was done in the C++ implementation, but in Rust we require submesh to be set in SubMorphTarget constructor
        // pCoreSubMorphTarget.setCoreSubmesh( this );

        return subMorphTargetId;
    }

    /*****************************************************************************/
    /** Returns the vertex vector.
     *
     * This function returns the vector that contains all vertices of the core
     * submesh instance.
     *
     * @return A reference to the vertex vector.
     *****************************************************************************/
    pub fn getVectorVertex(&self) -> &Vec<Vertex> {
        &self.m_vectorVertex
    }

    pub fn getVectorVertexMut(&mut self) -> &mut Vec<Vertex> {
        &mut self.m_vectorVertex
    }
}
