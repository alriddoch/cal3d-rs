use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use super::loader;
use super::skeleton::CalCoreSkeleton;

#[derive(Debug)]
pub enum CoreError {
    OtherError(String),
}

#[derive(Default)]
pub struct CalCoreModel {
    // std::string                           m_strName;
    pCoreSkeleton: Rc<RefCell<CalCoreSkeleton>>,
    // std::vector<CalCoreAnimationPtr>      m_vectorCoreAnimation;
    // std::vector<CalCoreAnimatedMorph *>   m_vectorCoreAnimatedMorph;

    // std::vector<CalCoreMeshPtr>           m_vectorCoreMesh;
    // std::vector<CalCoreMeshPtr>           m_vectorMorphMesh;
    // std::vector<CalCoreMaterialPtr>       m_vectorCoreMaterial;
    // std::map<int, std::map<int, int> >    m_mapmapCoreMaterialThread;
    // Cal::UserData                         m_userData;
    // std::map<std::string, int>            m_animationName;
    // std::map<std::string, int>            m_animatedMorphName;
    // std::map<std::string, int>            m_materialName;
    // std::map<std::string, int>            m_meshName;
    // unsigned int                          m_magic;
}

impl CalCoreModel {
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

        todo!();
        // add core animation to this core model
        //   return addCoreAnimation(pCoreAnimation.get());
        Ok(1)
    }

    pub fn loadCoreMesh(&mut self, filename: &PathBuf) -> Result<(), CoreError> {
        todo!();
        Ok(())
    }

    pub fn loadCoreMaterial(&mut self, filename: &PathBuf) -> Result<(), CoreError> {
        todo!();
        Ok(())
    }
}
