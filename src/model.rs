use super::CalMesh;
use super::CalMixer;
use crate::CalAbstractMixer;
use crate::CalMixerTrait;
use crate::CalMorphTargetMixer;
use crate::CalPhysique;
use crate::CalRenderer;
use crate::CalSkeleton;
use crate::CalSpringSystem;
use crate::core::CalCoreModel;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum ModelError {
    IndexError((usize, usize)),
}

pub struct CalModel {
    m_pCoreModel: Rc<RefCell<CalCoreModel>>,
    m_pSkeleton: Rc<RefCell<CalSkeleton>>,
    m_pMixer: CalAbstractMixer,
    m_pMorphTargetMixer: Option<CalMorphTargetMixer>,
    m_pPhysique: Option<CalPhysique>,
    m_pSpringSystem: Option<CalSpringSystem>,
    m_pRenderer: Option<CalRenderer>,
    m_userData: crate::UserData,
    /*  std::vector<CalMesh *> */
    m_vectorMesh: Vec<CalMesh>,
    // CalBoundingBox         m_boundingBox;
}

impl CalModel {
    pub fn new(core_model: Rc<RefCell<CalCoreModel>>) -> Self {
        let skeleton = core_model.borrow().getCoreSkeleton().clone();
        CalModel {
            m_pCoreModel: core_model,
            m_pSkeleton: Rc::new(RefCell::new(CalSkeleton::new(skeleton))),
            m_pMixer: CalAbstractMixer::None,
            m_pMorphTargetMixer: None,
            m_pPhysique: None,
            m_pSpringSystem: None,
            m_pRenderer: None,
            m_userData: Box::new(0),
            m_vectorMesh: Vec::new(),
        }
    }

    pub fn set_mixer(&mut self, mixer: CalMixer) {
        self.m_pMixer = CalAbstractMixer::CalMixer(mixer)
    }

    pub fn set_morph_target(&mut self, morph_target: CalMorphTargetMixer) {
        self.m_pMorphTargetMixer = Some(morph_target)
    }

    pub fn set_physique(&mut self, physique: CalPhysique) {
        self.m_pPhysique = Some(physique)
    }

    pub fn set_spring_system(&mut self, spring_system: CalSpringSystem) {
        self.m_pSpringSystem = Some(spring_system)
    }

    pub fn set_renderer(&mut self, renderer: CalRenderer) {
        self.m_pRenderer = Some(renderer)
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

    pub fn setMaterialSet(&mut self, setId: i32) {
        // set the lod level in all meshes

        for iteratorMesh in self.m_vectorMesh.iter_mut() {
            // set the material set in the mesh
            iteratorMesh.setMaterialSet(setId, &self.m_pCoreModel.borrow());
        }
    }

    pub fn getMixer(&self) -> Option<&CalMixer> {
        match &self.m_pMixer {
            CalAbstractMixer::CalMixer(mixer) => Some(mixer),
            _ => None,
        }
    }

    pub fn getMixerMut(&mut self) -> Option<&mut CalMixer> {
        match &mut self.m_pMixer {
            CalAbstractMixer::CalMixer(mixer) => Some(mixer),
            _ => None,
        }
    }

    // 188 cpp
    /*****************************************************************************/
    /** Provides access to the core model.
     *
     * This function returns the core model on which this model instance is based
     * on.
     *
     * @return One of the following values:
     *         \li a pointer to the core model
     *         \li \b 0 if an error happened
     *****************************************************************************/

    pub fn getCoreModel(&self) -> &Rc<RefCell<CalCoreModel>> {
        return &self.m_pCoreModel;
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

    // 700 cpp
    /*****************************************************************************/
    /** Updates the model instance.
     *
     * This function updates the model instance for a given amount of time.
     *
     * @param deltaTime The elapsed time in seconds since the last update.
     *****************************************************************************/
    pub fn update(&mut self, deltaTime: f32) {
        self.m_pMixer.updateAnimation(deltaTime);
        self.m_pMixer.updateSkeleton();
        // m_pMorpher.update(...);
        self.m_pMorphTargetMixer.as_mut().and_then(|m| {
            m.update(deltaTime);
            Option::<()>::None
        });
        self.m_pPhysique.as_mut().and_then(|m| {
            m.update();
            Option::<()>::None
        });
        self.m_pSpringSystem.as_mut().and_then(|m| {
            m.update(deltaTime);
            Option::<()>::None
        });
    }
}
