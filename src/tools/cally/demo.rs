use clap::Parser;

use super::menu::theMenu;
use super::model::*;
use super::graphics;

use std::{
    default,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum DemoError {
    ModelError(ModelError),
    PathError,
    OtherError(String),
}

impl From<ModelError> for DemoError {
    fn from(error: ModelError) -> Self {
        DemoError::ModelError(error)
    }
}

type Result<T> = std::result::Result<T, DemoError>;

impl From<graphics::GraphicsError> for DemoError {
    fn from(error: graphics::GraphicsError) -> Self {
        match error {
            graphics::GraphicsError::OtherError(e) => {
                DemoError::OtherError(e)
            }
        }
    }
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    fullscreen: bool,

    #[arg(long)]
    width: Option<u32>,

    #[arg(long)]
    height: Option<u32>,

    #[arg(long)]
    data: Option<String>,

    #[arg(long)]
    bench: bool,
}

// #[derive(Default)]
pub struct Demo {
    width: i32,
    height: i32,
    bFullscreen: bool,
    fpsDuration: f32,
    fpsFrames: i32,
    fps: i32,
    cursorTextureId: u32,
    logoTextureId: u32,
    fpsTextureId: u32,
    mouseX: i32,
    mouseY: i32,
    tiltAngle: f32,
    twistAngle: f32,
    distance: f32,
    bLeftMouseButtonDown: bool,
    bRightMouseButtonDown: bool,
    lastTick: u32,
    strDatapath: String,
    strCal3D_Datapath: String,
    vectorModel: Vec<Model>,
    currentModel: u32,
    bPaused: bool,
    averageCPUTime: f32,
    bOutputAverageCPUTimeAtExit: bool,

    screen: graphics::Screen,
    camera: graphics::Camera,
    tr: graphics::TextRenderer,
}

fn loadTexture(filename: &str) -> Result<u32> {
    // TODO: Copy from elsewhere
    // image::open(&Path::new(filename))
    Ok(1)
}

impl Demo {
    pub fn new() -> Result<Self> {
        Ok(Demo {
            width: 640,
            height: 480,
            bFullscreen: false,
            fpsDuration: 0.0,
            fpsFrames: 0,
            fps: 0,
            tiltAngle: -70.0,
            twistAngle: -45.0,
            distance: 270.0,
            strDatapath: String::from("data/"),
            screen: graphics::Screen::new("foo", 800, 600)?, 
            cursorTextureId: 0,
            logoTextureId: 0,
            fpsTextureId: 0,
            mouseX: 0,
            mouseY: 0,
            bLeftMouseButtonDown: false,
            bRightMouseButtonDown: false,
            lastTick: 0,
            strCal3D_Datapath: String::from(""),
            vectorModel: Vec::new(),
            currentModel: 0,
            bPaused: false,
            averageCPUTime: 0.0,
            bOutputAverageCPUTimeAtExit: false,
            camera: graphics::Camera::new(),
            tr: graphics::TextRenderer::new(),
            // ..Default::default()
        })
    }

    pub fn OnCreate(&mut self) -> Result<()> {
        println!(
            "o----------------------------------------------------------------o
|                      The Cally Demo 2.10.0                     |
|       (C) 2001, 2002, 2003 Bruno 'Beosil' Heidelberger         |
o----------------------------------------------------------------o
| This program is free software; you can redistribute it and/or  |
| modify it under the terms of the GNU General Public License as |
| published by the Free Software Foundation; either version 2 of |
| the License, or (at your option) any later version.            |
o----------------------------------------------------------------o"
        );

        let cli = Cli::parse();

        if cli.data.is_some() {
            self.strCal3D_Datapath = cli.data.unwrap();
        }

        // }
        Ok(())
    }

    pub fn OnInit(&mut self) -> Result<()> {
        // load the cursor texture
        let strFilename = [self.strDatapath.as_str(), "cursor.raw"]
            .iter()
            .collect::<PathBuf>();
        let strFilename = strFilename.to_str().ok_or(DemoError::PathError)?;

        self.cursorTextureId = loadTexture(strFilename)?;

        // load the logo texture
        let strFilename = [self.strDatapath.as_str(), "logo.raw"]
            .iter()
            .collect::<PathBuf>();
        let strFilename = strFilename.to_str().ok_or(DemoError::PathError)?;

        self.logoTextureId = loadTexture(strFilename)?;

        // load the fps texture
        let strFilename = [self.strDatapath.as_str(), "fps.raw"]
            .iter()
            .collect::<PathBuf>();
        let strFilename = strFilename.to_str().ok_or(DemoError::PathError)?;

        self.fpsTextureId = loadTexture(strFilename)?;

        // initialize models
        println!("Loading 'cally' model ...");

        let path = match self.strCal3D_Datapath.as_str() {
            "" => PathBuf::new(),
            _ => [self.strCal3D_Datapath.as_str(), "cally"]
                .iter()
                .collect::<PathBuf>(),
        };

        let mut pModel = Model::new(path);

        let cally_path = [self.strDatapath.as_str(), "cally.cfg"]
            .iter()
            .collect::<PathBuf>();
        let cally_path = cally_path.to_str().ok_or(DemoError::PathError)?;
        pModel.onInit(cally_path)?;

        self.vectorModel.push(pModel);

        println!("");

        // load 'skeleton' model
        println!("Loading 'skeleton' model ...");

        let path = match self.strCal3D_Datapath.as_str() {
            "" => PathBuf::new(),
            _ => [self.strCal3D_Datapath.as_str(), "skeleton"]
                .iter()
                .collect::<PathBuf>(),
        };

        let mut pModel = Model::new(path);

        let skeleton_path = [self.strDatapath.as_str(), "skeleton.cfg"]
            .iter()
            .collect::<PathBuf>();
        let skeleton_path = skeleton_path.to_str().ok_or(DemoError::PathError)?;
        pModel.onInit(skeleton_path)?;

        self.vectorModel.push(pModel);

        println!("");

        // load 'paladin' model
        println!("Loading 'paladin' model ...");

        let path = match self.strCal3D_Datapath.as_str() {
            "" => PathBuf::new(),
            _ => [self.strCal3D_Datapath.as_str(), "paladin"]
                .iter()
                .collect::<PathBuf>(),
        };

        let mut pModel = Model::new(path);

        let paladin_path = [self.strDatapath.as_str(), "paladin.cfg"]
            .iter()
            .collect::<PathBuf>();
        let paladin_path = paladin_path.to_str().ok_or(DemoError::PathError)?;
        pModel.onInit(paladin_path)?;

        self.vectorModel.push(pModel);

        println!("");

        // initialize menu
        theMenu.onInit(self.width, self.height);

        // we're done
        println!(
            "Initialization done.

Quit the demo by pressing 'q' or ESC
"
        );

        Ok(())
    }

    pub fn Loop(&mut self) {
        loop {
            // self.onIdle();

            // self.onRender();
            // self.onRenderInterface();
            self.screen.swap();

        }
    }
}
