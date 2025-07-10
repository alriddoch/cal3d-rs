use once_cell::sync::Lazy;

use crate::graphics::RendererError;
use crate::graphics::{LineRenderer, WithOrtho};
use crate::graphics::{Sprite, SpriteError};

use super::demo::*;
use super::graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::{
    default,
    path::{Path, PathBuf},
};

pub struct Menu {
    menuX: u32,
    menuY: u32,
    lodX: u32,
    lodY: u32,
    bMotionMovement: bool,
    bLodMovement: bool,
    bSkeleton: i32,
    bWireframe: bool,
    bLight: bool,
    actionTimespan: [f32; 2],
    nextTimespan: f32,
    theDemo: Option<Rc<RefCell<Demo>>>,

    sr: Option<Rc<RefCell<graphics::SpriteRenderer>>>,
    lr: graphics::LineRenderer,

    menu: Sprite,
    lod: Sprite,
}

pub enum MenuError {
    RendererError(RendererError),
    SpriteError(SpriteError),
    OtherError(String),
}

impl From<RendererError> for MenuError {
    fn from(error: RendererError) -> Self {
        MenuError::RendererError(error)
    }
}

impl From<SpriteError> for MenuError {
    fn from(error: SpriteError) -> Self {
        MenuError::SpriteError(error)
    }
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            bMotionMovement: false,
            menuX: 4,
            menuY: 4,
            bSkeleton: 0,
            bWireframe: false,
            bLight: true,
            actionTimespan: [0.0; 2],
            nextTimespan: 0.0,
            lodX: 4,
            lodY: 4,
            bLodMovement: false,
            theDemo: None,
            sr: None,
            lr: LineRenderer::new(),

            menu: Sprite::new(),
            lod: Sprite::new(),
        }
    }

    pub fn onInit(
        &mut self,
        demo: Rc<RefCell<Demo>>,
        sprite_renderer: Rc<RefCell<graphics::SpriteRenderer>>,
        width: u32,
        height: u32,
    ) -> Result<(), MenuError> {
        self.theDemo = Some(demo);
        self.sr = Some(sprite_renderer);

        // load the menu texture
        let strFilename = [
            self.theDemo.as_ref().unwrap().borrow().strDatapath.as_str(),
            "menu.raw",
        ]
        .iter()
        .collect::<PathBuf>();
        // filepath.Join(demo.borrow().strDatapath, "menu.raw")

        self.menu.WithSpriteFile(&strFilename).Setup()?;
        self.sr.as_ref().unwrap().borrow().bind(&self.menu);

        // load the lodxture
        let strFilename = [
            self.theDemo.as_ref().unwrap().borrow().strDatapath.as_str(),
            "lod.raw",
        ]
        .iter()
        .collect::<PathBuf>();

        self.lod.WithSpriteFile(&strFilename).Setup()?;
        self.sr.as_ref().unwrap().borrow().bind(&self.lod);

        self.lr.Setup(&WithOrtho(width, height))?;

        self.onResize(width, height);
        return Ok(());
    }

    // ----------------------------------------------------------------------------//
    // Update the menu                                                            //
    // ----------------------------------------------------------------------------//
    pub fn onUpdate(&mut self, elapsedSeconds: f32) {
        // calculate new timespan for f/x 1
        if self.actionTimespan[0] > 0.0 {
            self.actionTimespan[0] -= elapsedSeconds;
            if self.actionTimespan[0] < 0.0 {
                self.actionTimespan[0] = 0.0;
            }
        }

        // calculate new timespan for f/x 2
        if self.actionTimespan[1] > 0.0 {
            self.actionTimespan[1] -= elapsedSeconds;
            if self.actionTimespan[1] < 0.0 {
                self.actionTimespan[1] = 0.0;
            }
        }
        // calculate new timespan for 'next model'
        if self.nextTimespan > 0.0 {
            self.nextTimespan -= elapsedSeconds;
            if self.nextTimespan < 0.0 {
                self.nextTimespan = 0.0;
            }
        }
    }

    // ----------------------------------------------------------------------------//
    // Handle window resize event                                                 //
    // ----------------------------------------------------------------------------//
    // 429
    fn onResize(&mut self, width: u32, _height: u32) {
        // adjust menu position
        self.menuX = width - 132;

        // adjust lod position
        self.lodX = width / 2 - 128;
    }
}
