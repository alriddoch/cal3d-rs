use clap::Parser;

use super::graphics;
use super::menu::*;
use super::model::*;
use super::models::*;
use super::tick::*;
use crate::graphics::{Sprite, SpriteError};

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub enum DemoError {
    ModelError(ModelError),
    SpriteError(SpriteError),
    PathError,
    OtherError(String),
}

impl From<ModelError> for DemoError {
    fn from(error: ModelError) -> Self {
        DemoError::ModelError(error)
    }
}

impl From<SpriteError> for DemoError {
    fn from(error: SpriteError) -> Self {
        DemoError::SpriteError(error)
    }
}

type Result<T> = std::result::Result<T, DemoError>;

impl From<graphics::GraphicsError> for DemoError {
    fn from(error: graphics::GraphicsError) -> Self {
        match error {
            graphics::GraphicsError::OtherError(e) => DemoError::OtherError(e),
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
    width: u32,
    height: u32,
    bFullscreen: bool,
    fpsDuration: f32,
    fpsFrames: i32,
    fps: i32,
    mouseX: i32,
    mouseY: i32,
    tiltAngle: f32,
    twistAngle: f32,
    distance: f32,
    bLeftMouseButtonDown: bool,
    bRightMouseButtonDown: bool,
    lastTick: u128,
    pub strDatapath: String,
    strCal3D_Datapath: String,
    bPaused: bool,
    averageCPUTime: f32,
    bOutputAverageCPUTimeAtExit: bool,

    start: f64,
    firstTime: f64,
    lastTime: f64,
    bFirst: bool,
    cumul: f64,

    theModels: Rc<RefCell<Models>>,
    theMenu: Rc<RefCell<Menu>>,
    screen: graphics::Screen,
    camera: graphics::Camera,
    tr: graphics::TextRenderer,
    lr: graphics::LineRenderer,
    sr: Rc<RefCell<graphics::SpriteRenderer>>,

    cursor: Sprite,
    logo: Sprite,
    fps_sprite: Sprite,
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
            mouseX: 0,
            mouseY: 0,
            bLeftMouseButtonDown: false,
            bRightMouseButtonDown: false,
            lastTick: 0,
            strCal3D_Datapath: String::from(""),
            bPaused: false,
            averageCPUTime: 0.0,
            bOutputAverageCPUTimeAtExit: false,

            start: 0.0,
            firstTime: 0.0,
            lastTime: 0.0,
            bFirst: true,
            cumul: 0.0,
            theModels: Rc::new(RefCell::new(Models::new())),
            theMenu: Rc::new(RefCell::new(Menu::new())),
            camera: graphics::Camera::new(),
            tr: graphics::TextRenderer::new(),
            lr: graphics::LineRenderer::new(),
            sr: Rc::new(RefCell::new(graphics::SpriteRenderer::new())),

            cursor: Sprite::new(),
            logo: Sprite::new(),
            fps_sprite: Sprite::new(),
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
        self.camera
            .setup((self.width as f32) / (self.height as f32), 2000.0);

        self.tr.setup(self.width, self.height);
        // self.cr.Setup(self.camera);
        self.lr.Setup(graphics::WithCamera(&self.camera));
        self.sr.as_ref().borrow_mut().setup(self.width, self.height);

        // load the cursor texture
        let strFilename = [self.strDatapath.as_str(), "cursor.raw"]
            .iter()
            .collect::<PathBuf>();

        self.cursor.WithSpriteFile(&strFilename).Setup()?;

        // load the logo texture
        let strFilename = [self.strDatapath.as_str(), "logo.raw"]
            .iter()
            .collect::<PathBuf>();

        self.logo.WithSpriteFile(&strFilename).Setup()?;

        // load the fps texture
        let strFilename = [self.strDatapath.as_str(), "fps.raw"]
            .iter()
            .collect::<PathBuf>();

        self.fps_sprite.WithSpriteFile(&strFilename).Setup()?;

        self.theModels
            .borrow_mut()
            .init(self.strDatapath.as_str())?;

        // initialize menu
        self.theMenu.borrow_mut().onInit(
            self.theModels.clone(),
            self.sr.clone(),
            self.strDatapath.as_str(),
            self.width,
            self.height,
        );

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
            self.onIdle();

            // self.onRender();
            // self.onRenderInterface();
            self.screen.swap();
        }
    }

    fn onIdle(&mut self) {
        // get the current tick value
        let tick = getTick();

        // calculate the amount of elapsed seconds
        let elapsedSeconds = (tick - self.lastTick) as f32 / 1000.0;

        // adjust fps counter
        self.fpsDuration += elapsedSeconds;
        if self.fpsDuration >= 1.0 {
            self.fps = (self.fpsFrames as f32 / self.fpsDuration) as i32;
            self.fpsDuration = 0.0;
            self.fpsFrames = 0;
        }

        self.start = getTime();

        if self.bFirst {
            self.firstTime = self.start;
        } else {
            self.lastTime = self.start;
        }

        // update the current model
        if !self.bPaused {
            self.theModels.borrow_mut().idle(elapsedSeconds);
            //for (int i = 0; i < 10; i++)
        }

        let mut stop = getTime();

        stop -= self.start;

        self.cumul += stop;

        if !self.bFirst {
            self.averageCPUTime = (self.cumul / (self.lastTime - self.firstTime) * 100.0) as f32;
        }
        self.bFirst = false;

        // update the menu
        self.theMenu.borrow_mut().onUpdate(elapsedSeconds);

        // current tick will be last tick next round
        self.lastTick = tick;

        // update the screen
        //glutPostRedisplay()
    }
}
