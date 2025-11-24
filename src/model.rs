use super::CalMixer;
use std::{cell::RefCell, rc::Rc};

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
    // std::vector<CalMesh *> m_vectorMesh;
    // CalBoundingBox         m_boundingBox;
}

impl CalModel {
    pub fn new(core_model: Rc<RefCell<super::core::CalCoreModel>>) -> Self {
        CalModel {
            m_pCoreModel: core_model,
        }
    }

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
    pub fn attachMesh(&self, coreMeshId: u32) -> bool {
        todo!("model.cpp line 85");
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
