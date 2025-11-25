use super::CalMesh;
use super::CalMixer;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum ModelError {
    IndexError((usize, usize)),
}

#[derive(Default)]
pub struct CalModel {
    m_pCoreModel: Rc<RefCell<super::core::CalCoreModel>>,
    // CalSkeleton           *m_pSkeleton;
    // CalAbstractMixer      *m_pMixer;
    // CalMorphTargetMixer   *m_pMorphTargetMixer;
    // CalPhysique           *m_pPhysique;
    // CalSpringSystem       *m_pSpringSystem;
    // CalRenderer           *m_pRenderer;
    // Cal::UserData          m_userData;
    /*  std::vector<CalMesh *> */
    m_vectorMesh: Vec<CalMesh>,
    // CalBoundingBox         m_boundingBox;
}

impl CalModel {
    pub fn new(core_model: Rc<RefCell<super::core::CalCoreModel>>) -> Self {
        CalModel {
            m_pCoreModel: core_model,
            m_vectorMesh: Vec::new(),
        }
    }

    // 84 cpp
    /*****************************************************************************/
    /** Attachs a mesh.
     *
     * This function attachs a mesh to the model instance.
     *
     * @param coreMeshId The ID of the mesh that should be attached.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn attachMesh(&mut self, coreMeshId: usize) -> Result<(), ModelError> {
        let core_model = self.m_pCoreModel.borrow();

        // get the core mesh
        let pCoreMesh = core_model
            .getCoreMesh(coreMeshId)
            .ok_or(ModelError::IndexError((
                coreMeshId,
                core_model.getCoreMeshes().len(),
            )))?;

        // check if the mesh is already attached
        for meshId in self.m_vectorMesh.iter() {
            // check if we found the matching mesh
            if Rc::ptr_eq(meshId.getCoreMesh(), pCoreMesh) {
                // mesh is already active -> do nothing
                return Ok(());
            }
        }

        // allocate a new mesh instance
        let pMesh = CalMesh::new(pCoreMesh.clone());

        // set model in the mesh instance
        //pMesh.setModel(this);

        // insert the new mesh into the active list
        self.m_vectorMesh.push(pMesh);

        Ok(())
    }

    pub fn setMaterialSet(&self, setId: i32) {
        todo!("model.cpp line 669");
    }

    pub fn getMixer(&self) -> CalMixer {
        todo!("");
    }
    // CalModel(CalCoreModel *pCoreModel);
    // ~CalModel();

    // bool attachMesh(int coreMeshId);
    // bool detachMesh(int coreMeshId);
    // CalCoreModel *getCoreModel();
    // const CalCoreModel *getCoreModel() const;
    // CalMesh *getMesh(int coreMeshId);
    // const CalMesh *getMesh(int coreMeshId) const;
    // CalMixer *getMixer();
    // const CalMixer *getMixer() const;
    // const CalAbstractMixer *getAbstractMixer() const;
    // void setAbstractMixer(CalAbstractMixer *pMixer);
    // CalMorphTargetMixer *getMorphTargetMixer();
    // const CalMorphTargetMixer *getMorphTargetMixer() const;
    // CalPhysique *getPhysique();
    // const CalPhysique *getPhysique() const;
    // void setPhysique(CalPhysique *physique);
    // CalRenderer *getRenderer();
    // const CalRenderer *getRenderer() const;
    // CalSkeleton *getSkeleton();
    // const CalSkeleton *getSkeleton() const;
    // CalSpringSystem *getSpringSystem();
    // const CalSpringSystem *getSpringSystem() const;
    // CalBoundingBox & getBoundingBox(bool precision = false);
    // const Cal::UserData getUserData() const;
    // std::vector<CalMesh *>& getVectorMesh();
    // void setLodLevel(float lodLevel);
    // void setMaterialSet(int setId);
    // void setUserData(Cal::UserData userData);
    // void update(float deltaTime);
    // void disableInternalData();
}
