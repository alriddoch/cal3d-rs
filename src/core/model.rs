use std::cell::RefCell;
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
    // std::map<int, std::map<int, int> >    m_mapmapCoreMaterialThread;
    // Cal::UserData                         m_userData;
    // std::map<std::string, int>            m_animationName;
    // std::map<std::string, int>            m_animatedMorphName;
    // std::map<std::string, int>            m_materialName;
    // std::map<std::string, int>            m_meshName;
    // unsigned int                          m_magic;
}

impl CalCoreModel {
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
    fn addCoreAnimation(&mut self, pCoreAnimation: Rc<RefCell<CalCoreAnimation>>) -> i32 {
        let num = self.m_vectorCoreAnimation.len();

        // FIXME: Can Rc be null in Rust? No. Unclear if this is necessary for now.
        // for i in 0..num    {
        //   if !self.m_vectorCoreAnimation[ i ]       {
        //     self.m_vectorCoreAnimation[ i ] = pCoreAnimation;
        //     return i;
        //   }
        // }

        self.m_vectorCoreAnimation.push(pCoreAnimation);
        num as i32
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
    pub fn loadCoreAnimation(&mut self, filename: &PathBuf) -> Result<i32, loader::LoaderError> {
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
}
