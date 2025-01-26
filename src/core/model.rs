use std::path::PathBuf;

#[derive(Debug)]
pub enum CoreError {}

#[derive(Default)]
pub struct CalCoreModel {}

impl CalCoreModel {
    pub fn loadCoreSkeleton(&mut self, filename: &PathBuf) -> Result<(), CoreError> {
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
