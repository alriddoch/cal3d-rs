use super::graphics::get_texture;
use cal3d::{CalMixer, CalModel, CalMorphTargetMixer, CalPhysique, CalRenderer, CalSpringSystem};
use cgmath::Matrix4;
use std::ops::Deref;
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};

pub const STATE_IDLE: usize = 0;
pub const STATE_FANCY: usize = 1;
pub const STATE_MOTION: usize = 2;

#[derive(Default)]
pub struct Model {
    pub(crate) state: usize,
    calCoreModel: Rc<RefCell<cal3d::core::CalCoreModel>>,
    calModel: Option<Rc<RefCell<CalModel>>>,
    animationId: [usize; 16],
    animationCount: i32,
    meshId: [i32; 32],
    meshCount: i32,
    textureId: [u32; 32],
    textureCount: i32,
    pub(crate) motionBlend: [f32; 3],
    renderScale: f32,
    pub lodLevel: f32,
    path: PathBuf,
}

#[derive(Debug)]
pub(crate) enum ModelError {
    IoError(std::io::Error),
    ScanfError(sscanf::Error),
    ParseError(std::num::ParseFloatError),
    CoreError(cal3d::core::CoreError),
    CalModelError(cal3d::model::ModelError),
    SyntaxError,
    FormatError(String),
}

impl From<std::io::Error> for ModelError {
    fn from(error: std::io::Error) -> Self {
        ModelError::IoError(error)
    }
}

impl From<sscanf::Error> for ModelError {
    fn from(error: sscanf::Error) -> Self {
        ModelError::ScanfError(error)
    }
}

impl From<std::num::ParseFloatError> for ModelError {
    fn from(error: std::num::ParseFloatError) -> Self {
        ModelError::ParseError(error)
    }
}

impl From<cal3d::core::CoreError> for ModelError {
    fn from(error: cal3d::core::CoreError) -> Self {
        ModelError::CoreError(error)
    }
}

impl From<cal3d::model::ModelError> for ModelError {
    fn from(error: cal3d::model::ModelError) -> Self {
        ModelError::CalModelError(error)
    }
}

impl From<cal3d::core::LoaderError> for ModelError {
    fn from(error: cal3d::core::LoaderError) -> Self {
        match error {
            cal3d::core::LoaderError::IoError(e) => ModelError::IoError(e),
            cal3d::core::LoaderError::MagicError => {
                ModelError::FormatError(String::from("Incorrect magic number"))
            }
            cal3d::core::LoaderError::VersionError => {
                ModelError::FormatError(String::from("Invalid version number"))
            }
            cal3d::core::LoaderError::FormatError(e) => ModelError::FormatError(e),
        }
    }
}

impl Model {
    pub fn new(path: PathBuf) -> Self {
        Model {
            state: STATE_IDLE,
            motionBlend: [0.6, 0.1, 0.3],
            renderScale: 1.0,
            lodLevel: 1.0,
            path,
            ..Default::default()
        }
    }

    fn readFile(
        &mut self,
        name: &str,
        buff_reader: &mut std::io::BufReader<std::fs::File>,
    ) -> Result<(), ModelError> {
        use std::io::BufRead;

        let mut line = 0;

        let mut strPath = self.path.clone();

        let mut animationCount = 0;

        loop {
            line += 1;
            let mut buff = String::new();
            buff_reader.read_line(&mut buff)?;
            if buff.is_empty() {
                break;
            }
            if buff.ends_with('\n') {
                buff.pop();
            }
            println!("{}: '{}' ", buff.len(), buff);
            if buff.len() <= 1 || buff.starts_with('#') {
                println!("Skipping blank or comment line");
                continue;
            }

            let (key, value) = sscanf::scanf!(buff, r"{str}={str:/.+/}")?;

            match key {
                "scale" => {
                    self.renderScale = value.parse::<f32>()?;
                }
                "path" => {
                    if self.path.eq(&PathBuf::default()) {
                        strPath = PathBuf::from(value);
                    }
                }
                "skeleton" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.calCoreModel.borrow_mut().loadCoreSkeleton(&filename)?;
                }
                "animation" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.animationId[animationCount] = self
                        .calCoreModel
                        .borrow_mut()
                        .loadCoreAnimation(&filename)?;
                    animationCount += 1;
                }
                "mesh" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.calCoreModel.borrow_mut().loadCoreMesh(&filename)?;
                }
                "material" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.calCoreModel.borrow_mut().loadCoreMaterial(&filename)?;
                }
                _ => {
                    println!("{name}({line}): Invalid syntax.");
                    return Err(ModelError::SyntaxError);
                }
            }
        }
        Ok(())
    }

    pub fn onInit(&mut self, filename: &str) -> Result<(), ModelError> {
        use std::fs;
        use std::io::BufReader;
        println!("Opening path: {filename}");
        let mut buff_reader = BufReader::new(fs::File::open(filename)?);
        self.readFile(filename, &mut buff_reader)?;

        let strPath = self.path.clone();

        let mut core_model = self.calCoreModel.borrow_mut();
        let core_materials = core_model.getCoreMaterialsMut();
        // load all textures and store the opengl texture id in the corresponding map in the material
        //   int materialId;

        // get the core material

        for pCoreMaterial in core_materials.iter_mut() {
            let material_maps = pCoreMaterial.getMapsMut();
            // loop through all maps of the core material

            for map in material_maps.iter_mut() {
                // get the filename of the texture

                let mut filename = strPath.clone();
                filename.push(&map.strFilename);

                // load the texture from the file
                let textureId = get_texture(&filename, false, 1)?;

                // store the opengl texture id in the user data of the map
                map.userData = textureId as i32;
            }
        }

        // make one material thread for each material
        // NOTE: this is not the right way to do it, but this viewer can't do the right
        // mapping without further information on the model etc.
        for materialId in 0..core_materials.len() as i32 {
            // create the a material thread
            core_model.createCoreMaterialThread(materialId);

            // initialize the material thread
            core_model.setCoreMaterialId(materialId, 0, materialId);
        }

        // Drop mut borrow, as not-mut borrow must be possible below.
        drop(core_model);

        // Calculate Bounding Boxes
        self.calCoreModel
            .borrow()
            .getCoreSkeleton()
            .borrow()
            .calculateBoundingBoxes(&self.calCoreModel);

        let mut cal_model = CalModel::new(self.calCoreModel.clone());

        // attach all meshes to the model
        // TODO Can we just iterate over the coreMeshes?
        for meshId in 0..self.calCoreModel.borrow().getCoreMeshes().len() {
            cal_model.attachMesh(meshId)?;
        }

        // set the material set of the whole model
        cal_model.setMaterialSet(0);

        let cal_model = Rc::new(RefCell::new(cal_model));
        let mut cal_mixer = CalMixer::new(cal_model.clone());
        let core_model = self.calCoreModel.borrow();

        // set initial animation state
        self.state = STATE_MOTION;
        cal_mixer.blendCycle(
            core_model.deref(),
            self.animationId[STATE_MOTION],
            self.motionBlend[0],
            0.0,
        );
        cal_mixer.blendCycle(
            core_model.deref(),
            self.animationId[STATE_MOTION + 1],
            self.motionBlend[1],
            0.0,
        );
        cal_mixer.blendCycle(
            core_model.deref(),
            self.animationId[STATE_MOTION + 2],
            self.motionBlend[2],
            0.0,
        );

        // Delay setting mixer on CalModel until after borrows above, as CalModel is borrowed in blendCycle
        cal_model.borrow_mut().set_mixer(cal_mixer);
        cal_model
            .borrow_mut()
            .set_morph_target(CalMorphTargetMixer::new(cal_model.clone()));
        cal_model
            .borrow_mut()
            .set_physique(CalPhysique::new(cal_model.clone()));
        cal_model
            .borrow_mut()
            .set_spring_system(CalSpringSystem::new(cal_model.clone()));
        cal_model
            .borrow_mut()
            .set_renderer(CalRenderer::new(cal_model.clone()));

        self.calModel = Some(cal_model);

        Ok(())
    }

    //517.go
    // ----------------------------------------------------------------------------//
    // Update the model                                                           //
    // ----------------------------------------------------------------------------//
    pub fn onUpdate(&mut self, elapsedSeconds: f32) {
        if self.calModel.is_none() {
            println!("No calmodel. Skipping update ...");
            return;
        }
        // update the model
        self.calModel
            .as_ref()
            .unwrap()
            .borrow_mut()
            .update(elapsedSeconds)
    }

    pub fn getRenderScale(&self) -> f32 {
        self.renderScale
    }

    pub fn render(&self, view: &Matrix4<f32>) {
        todo!("Unable to render yet");
    }

    // 862
    // ----------------------------------------------------------------------------//
    // Set the motion blend factors state of the model                            //
    // ----------------------------------------------------------------------------//
    pub fn setMotionBlend(&mut self, pMotionBlend: [f32; 3], delay: f32) {
        self.motionBlend[0] = pMotionBlend[0];
        self.motionBlend[1] = pMotionBlend[1];
        self.motionBlend[2] = pMotionBlend[2];

        let Some(mut cal_model) = self.calModel.as_ref().map(|m| m.borrow_mut()) else {
            panic!("Unable to get CalModel");
            return;
        };

        // FIXME Use self.calCoreModel instead?
        // let core_model_ref = cal_model.getCoreModel().clone();
        // let core_model = core_model_ref.borrow();
        let core_model = self.calCoreModel.borrow();

        let mixer = cal_model.getMixerMut().expect("CalModel has no mixer");

        mixer.clearCycle(self.animationId[STATE_IDLE], delay);
        mixer.clearCycle(self.animationId[STATE_FANCY], delay);
        mixer.blendCycle(
            core_model.deref(),
            self.animationId[STATE_MOTION],
            self.motionBlend[0],
            delay,
        );
        mixer.blendCycle(
            core_model.deref(),
            self.animationId[STATE_MOTION + 1],
            self.motionBlend[1],
            delay,
        );
        mixer.blendCycle(
            core_model.deref(),
            self.animationId[STATE_MOTION + 2],
            self.motionBlend[2],
            delay,
        );

        self.state = STATE_MOTION
    }

    //----------------------------------------------------------------------------//
    // Set a new animation state within a given delay                             //
    //----------------------------------------------------------------------------//
    pub fn setState(&mut self, state: usize, delay: f32) {
        // todo!();
        // check if this is really a new state
        let Some(mut cal_model) = self.calModel.as_ref().map(|m| m.borrow_mut()) else {
            panic!("Unable to get CalModel");
            return;
        };

        let core_model = self.calCoreModel.borrow();
        let mixer = cal_model.getMixerMut().expect("CalModel has no mixer");
        if state != self.state {
            if state == STATE_IDLE {
                mixer.blendCycle(core_model.deref(), self.animationId[STATE_IDLE], 1.0, delay);
                mixer.clearCycle(self.animationId[STATE_FANCY], delay);
                mixer.clearCycle(self.animationId[STATE_MOTION], delay);
                mixer.clearCycle(self.animationId[STATE_MOTION + 1], delay);
                mixer.clearCycle(self.animationId[STATE_MOTION + 2], delay);
                self.state = STATE_IDLE
            } else if state == STATE_FANCY {
                mixer.clearCycle(self.animationId[STATE_IDLE], delay);
                mixer.blendCycle(
                    core_model.deref(),
                    self.animationId[STATE_FANCY],
                    1.0,
                    delay,
                );
                mixer.clearCycle(self.animationId[STATE_MOTION], delay);
                mixer.clearCycle(self.animationId[STATE_MOTION + 1], delay);
                mixer.clearCycle(self.animationId[STATE_MOTION + 2], delay);
                self.state = STATE_FANCY
            } else if state == STATE_MOTION {
                mixer.clearCycle(self.animationId[STATE_IDLE], delay);
                mixer.clearCycle(self.animationId[STATE_FANCY], delay);
                mixer.blendCycle(
                    core_model.deref(),
                    self.animationId[STATE_MOTION],
                    self.motionBlend[0],
                    delay,
                );
                mixer.blendCycle(
                    core_model.deref(),
                    self.animationId[STATE_MOTION + 1],
                    self.motionBlend[1],
                    delay,
                );
                mixer.blendCycle(
                    core_model.deref(),
                    self.animationId[STATE_MOTION + 2],
                    self.motionBlend[2],
                    delay,
                );
                self.state = STATE_MOTION
            }
        }
        self.state = state;
    }
}
