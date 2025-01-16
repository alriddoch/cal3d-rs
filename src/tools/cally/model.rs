use std::path::PathBuf;

#[derive(Default)]
pub struct Model {
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
