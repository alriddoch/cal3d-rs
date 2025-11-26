use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use super::animation::CalCoreAnimation;
use super::loader;
use super::material::CalCoreMaterial;
use super::mesh::CalCoreMesh;
use super::skeleton::CalCoreSkeleton;

#[derive(Debug)]
pub enum CoreError {
    OtherError(String),
}

#[derive(Default)]
pub struct CalCoreModel {
    // std::string                           m_strName;
    pCoreSkeleton: Rc<RefCell<CalCoreSkeleton>>,
    m_vectorCoreAnimation: Vec<Rc<RefCell<CalCoreAnimation>>>,
    // std::vector<CalCoreAnimatedMorph *>   m_vectorCoreAnimatedMorph;
    m_vectorCoreMesh: Vec<Rc<RefCell<CalCoreMesh>>>,
    // std::vector<CalCoreMeshPtr>           m_vectorMorphMesh;
    m_vectorCoreMaterial: Vec<CalCoreMaterial>,
    m_mapmapCoreMaterialThread: BTreeMap<i32, BTreeMap<i32, i32>>,
    // Cal::UserData                         m_userData;
    // std::map<std::string, int>            m_animationName;
    // std::map<std::string, int>            m_animatedMorphName;
    // std::map<std::string, int>            m_materialName;
    // std::map<std::string, int>            m_meshName;
    // unsigned int                          m_magic;
}

impl CalCoreModel {
    pub fn getCoreSkeleton(&self) -> &Rc<RefCell<CalCoreSkeleton>> {
        &self.pCoreSkeleton
    }

    pub fn getCoreMaterials(&self) -> &Vec<CalCoreMaterial> {
        &self.m_vectorCoreMaterial
    }

    pub fn getCoreMaterialsMut(&mut self) -> &mut Vec<CalCoreMaterial> {
        &mut self.m_vectorCoreMaterial
    }

    pub fn getCoreMeshes(&self) -> &Vec<Rc<RefCell<CalCoreMesh>>> {
        &self.m_vectorCoreMesh
    }

    pub fn getCoreAnimationCount(&self) -> usize {
        return self.m_vectorCoreAnimation.len();
    }

    //112
    /*****************************************************************************/
    /** Adds a core animation.
     *
     * This function adds a core animation to the core model instance.
     *
     * @param pCoreAnimation A pointer to the core animation that should be added.
     *
     * @return \li the assigned animation \b ID of the added core animation
     *****************************************************************************/
    fn addCoreAnimation(&mut self, pCoreAnimation: Rc<RefCell<CalCoreAnimation>>) -> usize {
        let num = self.m_vectorCoreAnimation.len();

        // FIXME: Can Rc be null in Rust? No. Unclear if this is necessary for now.
        // for i in 0..num    {
        //   if !self.m_vectorCoreAnimation[ i ]       {
        //     self.m_vectorCoreAnimation[ i ] = pCoreAnimation;
        //     return i;
        //   }
        // }

        self.m_vectorCoreAnimation.push(pCoreAnimation);
        num
    }

    //278
    /*****************************************************************************/
    /** Adds a core material.
     *
     * This function adds a core material to the core model instance.
     *
     * @param pCoreMaterial A pointer to the core material that should be added.
     *
     * @return One of the following values:
     *         \li the assigned material \b ID of the added core material
     *         \li \b -1 if an error happened
     *****************************************************************************/
    pub fn addCoreMaterial(&mut self, pCoreMaterial: CalCoreMaterial) -> i32 {
        let num = self.m_vectorCoreMaterial.len();

        // FIXME: Need a way to work out if replacing is needed
        //   for  i in 0..num  {
        //      if !self.m_vectorCoreMaterial[ i ]      {
        //         self.m_vectorCoreMaterial[ i ] = pCoreMaterial;
        //         return i;
        //      }
        //   }

        self.m_vectorCoreMaterial.push(pCoreMaterial);
        num as i32
    }

    //325
    /*****************************************************************************/
    /** Adds a core mesh.
     *
     * This function adds a core mesh to the core model instance.
     *
     * @param pCoreMesh A pointer to the core mesh that should be added.
     *
     * @return One of the following values:
     *         \li the assigned mesh \b ID of the added core material
     *         \li \b -1 if an error happened
     *****************************************************************************/
    pub fn addCoreMesh(&mut self, pCoreMesh: Rc<RefCell<CalCoreMesh>>) -> usize {
        let num = self.m_vectorCoreMesh.len();

        // FIXME: Can Rc be null in Rust? No. Unclear if this is necessary for now.
        //   for  i in 0..num  {
        //      if( !self.m_vectorCoreMesh[ i ] )     {
        //         self.m_vectorCoreMesh[ i ] = pCoreMesh;
        //         return i;
        //      }
        //   }

        self.m_vectorCoreMesh.push(pCoreMesh);
        return num;
    }

    // 372 cpp
    /*****************************************************************************/
    /** Creates a core material thread.
     *
     * This function creates a new core material thread with the given ID.
     *
     * @param coreMaterialThreadId The ID of the core material thread that should
     *                             be created.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn createCoreMaterialThread(&mut self, coreMaterialThreadId: i32) -> bool {
        // insert an empty core material thread with a given id
        let mapCoreMaterialThreadId = BTreeMap::<i32, i32>::default();
        self.m_mapmapCoreMaterialThread
            .insert(coreMaterialThreadId, mapCoreMaterialThreadId);

        return true;
    }

    // 393
    /*****************************************************************************/
    /** Provides access to a core animation.
     *
     * This function returns the core animation with the given ID.
     *
     * @param coreAnimationId The ID of the core animation that should be returned.
     *
     * @return One of the following values:
     *         \li a pointer to the core animation
     *         \li \b 0 if an error happened
     *****************************************************************************/
    pub fn getCoreAnimation(
        &self,
        coreAnimationId: usize,
    ) -> Option<&Rc<RefCell<CalCoreAnimation>>> {
        return self.m_vectorCoreAnimation.get(coreAnimationId);
    }

    /*****************************************************************************/
    /** Provides access to a core mesh.
     *
     * This function returns the core mesh with the given ID.
     *
     * @param coreMeshId The ID of the core mesh that should be returned.
     *
     * @return One of the following values:
     *         \li a pointer to the core mesh
     *         \li \b 0 if an error happened
     *****************************************************************************/
    pub fn getCoreMesh(&self, coreMeshId: usize) -> Option<&Rc<RefCell<CalCoreMesh>>> {
        self.m_vectorCoreMesh.get(coreMeshId)
    }

    /*****************************************************************************/
    /** Returns a specified core material ID.
     *
     * This function returns the core material ID for a specified core material
     * thread / core material set pair.
     *
     * @param coreMaterialThreadId The ID of the core material thread.
     * @param coreMaterialSetId The ID of the core material set.
     *
     * @return One of the following values:
     *         \li the \b ID of the core material
     *         \li \b -1 if an error happened
     *****************************************************************************/

    pub fn getCoreMaterialId(
        &self,
        coreMaterialThreadId: i32,
        coreMaterialSetId: i32,
    ) -> Option<i32> {
        // get the core material thread
        let Some(coreMaterialThread) = self.m_mapmapCoreMaterialThread.get(&coreMaterialThreadId)
        else {
            // CalError::setLastError(CalError::INVALID_HANDLE, __FILE__, __LINE__);
            return None;
        };

        // find the material id for the given set
        coreMaterialThread.get(&coreMaterialSetId).copied()
    }

    // 635 cpp
    /*****************************************************************************/
    /** Returns the number of core meshes.
     *
     * This function returns the number of core meshes in the core model instance.
     *
     * @return The number of core meshes.
     *****************************************************************************/
    pub fn getCoreMeshCount(&self) -> usize {
        self.m_vectorCoreMesh.len()
    }

    //659
    /*****************************************************************************/
    /** Loads a core animation.
     *
     * This function loads a core animation from a file.
     *
     * @param strFilename The file from which the core animation should be loaded
     *                    from.
     *
     * @return One of the following values:
     *         \li the assigned \b ID of the loaded core animation
     *         \li \b -1 if an error happened
     *****************************************************************************/
    pub fn loadCoreAnimation(&mut self, filename: &PathBuf) -> Result<usize, loader::LoaderError> {
        // FIXME Check if skeleton has been loaded.
        // the core skeleton has to be loaded already
        //   if(!m_pCoreSkeleton)  {
        //     CalError::setLastError(CalError::INVALID_HANDLE, __FILE__, __LINE__);
        //     return -1;
        //   }

        // load a new core animation
        let pCoreAnimation = loader::loadCoreAnimation(filename, &self.pCoreSkeleton)?;

        // add core animation to this core model
        Ok(self.addCoreAnimation(pCoreAnimation))
    }

    //1016
    /*****************************************************************************/
    /** Loads a core material.
     *
     * This function loads a core material from a file.
     *
     * @param strFilename The file from which the core material should be loaded
     *                    from.
     *
     * @return One of the following values:
     *         \li the assigned \b ID of the loaded core material
     *         \li \b -1 if an error happened
     *****************************************************************************/
    pub fn loadCoreMaterial(&mut self, filename: &PathBuf) -> Result<i32, loader::LoaderError> {
        // FIXME Check if skeleton has been loaded.
        // the core skeleton has to be loaded already
        //   if(!m_pCoreSkeleton)  {
        //     CalError::setLastError(CalError::INVALID_HANDLE, __FILE__, __LINE__);
        //     return -1;
        //   }

        let pCoreMaterial = loader::loadCoreMaterial(filename)?;

        Ok(self.addCoreMaterial(pCoreMaterial))
    }

    //1211
    /*****************************************************************************/
    /** Loads a core mesh.
     *
     * This function loads a core mesh from a file.
     *
     * @param strFilename The file from which the core mesh should be loaded from.
     *
     * @return One of the following values:
     *         \li the assigned \b ID of the loaded core mesh
     *         \li \b -1 if an error happened
     *****************************************************************************/
    pub fn loadCoreMesh(&mut self, filename: &PathBuf) -> Result<usize, loader::LoaderError> {
        // FIXME Check if skeleton has been loaded.
        // the core skeleton has to be loaded already
        //   if(!m_pCoreSkeleton)  {
        //     CalError::setLastError(CalError::INVALID_HANDLE, __FILE__, __LINE__);
        //     return -1;
        //   }

        let pCoreMesh = loader::loadCoreMesh(filename)?;

        Ok(self.addCoreMesh(pCoreMesh))
    }

    //1404
    /*****************************************************************************/
    /** Loads the core skeleton.
     *
     * This function loads the core skeleton from a file.
     *
     * @param strFilename The file from which the core skeleton should be loaded
     *                    from.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn loadCoreSkeleton(&mut self, filename: &PathBuf) -> Result<(), loader::LoaderError> {
        loader::loadCoreSkeleton(filename, &self.pCoreSkeleton)?;
        Ok(())
    }

    // 1591 cpp
    /*****************************************************************************/
    /** Sets a core material ID.
     *
     * This function sets a core material ID for a core material thread / core
     * material set pair.
     *
     * @param coreMaterialThreadId The ID of the core material thread.
     * @param coreMaterialSetId The ID of the core maetrial set.
     * @param coreMaterialId The ID of the core maetrial.
     *
     * @return One of the following values:
     *         \li \b true if successful
     *         \li \b false if an error happened
     *****************************************************************************/
    pub fn setCoreMaterialId(
        &mut self,
        coreMaterialThreadId: i32,
        coreMaterialSetId: i32,
        coreMaterialId: i32,
    ) -> bool {
        // find the core material thread
        let Some(coreMaterialThread) = self
            .m_mapmapCoreMaterialThread
            .get_mut(&coreMaterialThreadId)
        else {
            // CalError::setLastError(CalError::INVALID_HANDLE, __FILE__, __LINE__);
            return false;
        };

        // remove a possible entry in the core material thread
        coreMaterialThread.remove(&coreMaterialSetId);

        // set the given set id in the core material thread to the given core material id
        coreMaterialThread.insert(coreMaterialSetId, coreMaterialId);

        return true;
    }
}
