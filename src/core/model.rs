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
    pCoreSkeleton: Rc<CalCoreSkeleton>,
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
    pub fn loadCoreSkeleton(&mut self, filename: &PathBuf) -> Result<(), loader::LoaderError> {
        loader::loadCoreSkeleton(filename, &self.pCoreSkeleton)?;
        // FIXME Implement
        Ok(())
    }

    pub fn loadCoreAnimation(&mut self, filename: &PathBuf) -> Result<i32, CoreError> {
        // FIXME Implement
        Ok(1)
    }

    pub fn loadCoreMesh(&mut self, filename: &PathBuf) -> Result<(), CoreError> {
        // FIXME Implement
        Ok(())
    }

    pub fn loadCoreMaterial(&mut self, filename: &PathBuf) -> Result<(), CoreError> {
        // FIXME Implement
        Ok(())
    }
}
