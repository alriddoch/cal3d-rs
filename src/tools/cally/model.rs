use cgmath::Matrix4;
use std::path::PathBuf;

pub const STATE_IDLE: usize = 0;
pub const STATE_FANCY: usize = 1;
pub const STATE_MOTION: usize = 2;

#[derive(Default)]
pub struct Model {
    pub(crate) state: usize,
    calCoreModel: cal3d::core::CalCoreModel,
    calModel: cal3d::CalModel,
    animationId: [i32; 16],
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
                    self.calCoreModel.loadCoreSkeleton(&filename)?;
                }
                "animation" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.animationId[animationCount] =
                        self.calCoreModel.loadCoreAnimation(&filename)?;
                    animationCount += 1;
                }
                "mesh" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.calCoreModel.loadCoreMesh(&filename)?;
                }
                "material" => {
                    let mut filename = strPath.clone();
                    filename.push(value);
                    self.calCoreModel.loadCoreMaterial(&filename)?;
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

        // FIXME: Postprocessing, textures, etc.
        Ok(())
    }

    //517.go
    // ----------------------------------------------------------------------------//
    // Update the model                                                           //
    // ----------------------------------------------------------------------------//
    pub fn onUpdate(&mut self, elapsedSeconds: f32) {
        // if s.calModel == nil {
        //     // fmt.Printf("No calmodel. Skipping update ...\n")
        //     return;
        // }
        // // update the model
        // s.calModel.Update(elapsedSeconds)
    }

    pub fn getRenderScale(&self) -> f32 {
        self.renderScale
    }

    pub fn render(&self, view: &Matrix4<f32>) {
        unimplemented!();
    }

    // 862
    // ----------------------------------------------------------------------------//
    // Set the motion blend factors state of the model                            //
    // ----------------------------------------------------------------------------//
    pub fn setMotionBlend(&mut self, pMotionBlend: [f32; 3], delay: f32) {
        self.motionBlend[0] = pMotionBlend[0];
        self.motionBlend[1] = pMotionBlend[1];
        self.motionBlend[2] = pMotionBlend[2];

        unimplemented!();

        // self.calModel
        //     .GetMixer()
        //     .ClearCycle(self.animationId[STATE_IDLE], delay);
        // self.calModel
        //     .GetMixer()
        //     .ClearCycle(self.animationId[STATE_FANCY], delay);
        // self.calModel.GetMixer().BlendCycle(
        //     self.animationId[STATE_MOTION],
        //     self.motionBlend[0],
        //     delay,
        // );
        // self.calModel.GetMixer().BlendCycle(
        //     self.animationId[STATE_MOTION + 1],
        //     self.motionBlend[1],
        //     delay,
        // );
        // self.calModel.GetMixer().BlendCycle(
        //     self.animationId[STATE_MOTION + 2],
        //     self.motionBlend[2],
        //     delay,
        // );

        self.state = STATE_MOTION
    }
}
