use cgmath::Matrix4;
use std::path::PathBuf;

use super::demo::DemoError;
use super::model::*;

pub(crate) struct Models {
    vectorModel: Vec<Model>,
    currentModel: usize,
}

impl Models {
    pub fn new() -> Self {
        Models {
            vectorModel: Vec::new(),
            currentModel: 0,
        }
    }

    pub(crate) fn init(&mut self, data_path: &str) -> Result<(), DemoError> {
        // initialize models
        println!("Loading 'cally' model ...");

        let path = match data_path {
            "" => PathBuf::new(),
            _ => [data_path, "cally"].iter().collect::<PathBuf>(),
        };

        println!("");

        let mut pModel = Model::new(path);

        let cally_path = [data_path, "cally.cfg"].iter().collect::<PathBuf>();
        let cally_path = cally_path.to_str().ok_or(DemoError::PathError)?;
        pModel.onInit(cally_path)?;

        self.vectorModel.push(pModel);

        println!("");

        // load 'skeleton' model
        println!("Loading 'skeleton' model ...");

        let path = match data_path {
            "" => PathBuf::new(),
            _ => [data_path, "skeleton"].iter().collect::<PathBuf>(),
        };

        let mut pModel = Model::new(path);

        let skeleton_path = [data_path, "skeleton.cfg"].iter().collect::<PathBuf>();
        let skeleton_path = skeleton_path.to_str().ok_or(DemoError::PathError)?;
        pModel.onInit(skeleton_path)?;

        self.vectorModel.push(pModel);

        println!("");

        // load 'paladin' model
        println!("Loading 'paladin' model ...");

        let path = match data_path {
            "" => PathBuf::new(),
            _ => [data_path, "paladin"].iter().collect::<PathBuf>(),
        };

        let mut pModel = Model::new(path);

        let paladin_path = [data_path, "paladin.cfg"].iter().collect::<PathBuf>();
        let paladin_path = paladin_path.to_str().ok_or(DemoError::PathError)?;
        pModel.onInit(paladin_path)?;

        self.vectorModel.push(pModel);

        Ok(())
    }

    pub(crate) fn idle(&mut self, elapsedSeconds: f32) {
        self.vectorModel[self.currentModel].onUpdate(elapsedSeconds);
    }

    pub fn render_scale(&self) -> f32 {
        self.vectorModel[self.currentModel].getRenderScale()
    }

    pub fn render(&self, view: &Matrix4<f32>) {
        self.vectorModel[self.currentModel].render(view)
    }

    pub fn get_model_state(&self) -> u32 {
        self.vectorModel[self.currentModel].state
    }

    pub fn getMotionBlend(&self) -> &[f32;3] {
        &self.vectorModel[self.currentModel].motionBlend
    }
}
