use std::path::PathBuf;

#[derive(Default)]
pub struct Model {
     m_state: i32,
    calCoreModel: cal3d::core::CalCoreModel,
    calModel: cal3d::CalModel,
    animationId: [i32; 16],
    animationCount: i32,
    meshId: [i32; 32],
    meshCount: i32,
    textureId: [u32; 32],
    textureCount: i32,
    motionBlend: [f32; 3],
    renderScale: f32,
    lodLevel: f32,
    path: PathBuf,
}

impl Model {
    pub fn new(path: PathBuf) -> Self {
        Model{
            path,
            ..Default::default()
        }
    }

    pub fn onInit(&mut self, filename: &str) -> Result<(), ()> {
        Ok(())
    }
}
