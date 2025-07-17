use cgmath::Matrix4;
use cgmath::SquareMatrix;
use cgmath::Vector3;
use cgmath::{Deg, Rad};
use glfw::{Action, Key};
use std::cell::RefCell;
use std::ops::Mul;
use std::path::PathBuf;
use std::rc::Rc;

use crate::graphics::RendererError;
use crate::graphics::{LineRenderer, WithOrtho};
use crate::graphics::{Sprite, SpriteError};

use super::graphics;
use super::models::*;

pub struct Menu {
    menuX: i32,
    menuY: i32,
    lodX: u32,
    lodY: u32,
    bMotionMovement: bool,
    bLodMovement: bool,
    bSkeleton: i32,
    bWireframe: bool,
    bLight: bool,
    actionTimespan: [f32; 2],
    nextTimespan: f32,
    theModels: Option<Rc<RefCell<Models>>>,

    sr: Option<Rc<RefCell<graphics::SpriteRenderer>>>,
    lr: graphics::LineRenderer,

    menu: Sprite,
    lod: Sprite,
}

#[derive(Debug)]
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

const MENUITEM_Y: [i32; 5] = [228, 200, 94, 66, 38];
const MENUITEM_HEIGHT: [i32; 5] = [28, 28, 106, 28, 28];
const MENUITEM_MOTION_X: [i32; 3] = [42, 80, 118];
const MENUITEM_MOTION_Y: [i32; 3] = [168, 102, 168];

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
            theModels: None,
            sr: None,
            lr: LineRenderer::new(),

            menu: Sprite::new(),
            lod: Sprite::new(),
        }
    }

    pub fn onInit(
        &mut self,
        models: Rc<RefCell<Models>>,
        sprite_renderer: Rc<RefCell<graphics::SpriteRenderer>>,
        path: &str,
        width: u32,
        height: u32,
    ) -> Result<(), MenuError> {
        self.theModels = Some(models);
        self.sr = Some(sprite_renderer);

        // load the menu texture
        let strFilename = [path, "menu.raw"].iter().collect::<PathBuf>();
        // filepath.Join(demo.borrow().strDatapath, "menu.raw")

        self.menu.WithSpriteFile(&strFilename).Setup()?;
        self.sr.as_ref().unwrap().borrow().bind(&self.menu);

        // load the lodxture
        let strFilename = [path, "lod.raw"].iter().collect::<PathBuf>();

        self.lod.WithSpriteFile(&strFilename).Setup()?;
        self.sr.as_ref().unwrap().borrow().bind(&self.lod);

        self.lr.Setup(&WithOrtho(width, height))?;

        self.onResize(width, height);
        return Ok(());
    }

    pub fn onRender(&self) {
        let sr = self.sr.as_ref().unwrap().borrow();
        sr.set_state();

        let mview = Matrix4::<f32>::identity().mul(Matrix4::from_translation(Vector3 {
            x: self.menuX as f32,
            y: self.menuY as f32,
            z: 0.0,
        }));

        sr.set_sprite(&self.menu);
        self.menu.set_area(0, 0, 128, 256, 128, 0);
        sr.draw(&mview);

        let state = self.theModels.as_ref().unwrap().borrow().get_model_state();

        let startY = MENUITEM_Y[state as usize];
        let diffY = MENUITEM_HEIGHT[state as usize];

        self.menu
            .set_area(0, startY as u32, 128, diffY as u32, 0, 0);
        sr.draw(&mview);

        if self.actionTimespan[0] > 0.0 {
            let startY = MENUITEM_Y[3];
            let diffY = MENUITEM_HEIGHT[3];
            self.menu
                .set_area(0, startY as u32, 128, diffY as u32, 0, 0);
            sr.draw(&mview);
        }

        if self.actionTimespan[1] > 0.0 {
            let startY = MENUITEM_Y[4];
            let diffY = MENUITEM_HEIGHT[4];
            self.menu
                .set_area(0, startY as u32, 128, diffY as u32, 0, 0);
            sr.draw(&mview);
        }

        if self.bSkeleton != 0 {
            self.menu.set_area(0, 0, 32, 35, 0, 0);
            sr.draw(&mview);
        }

        if self.bWireframe {
            self.menu.set_area(32, 0, 32, 35, 0, 0);
            sr.draw(&mview);
        }

        if self.bLight {
            self.menu.set_area(64, 0, 32, 35, 0, 0);
            sr.draw(&mview);
        }

        if self.nextTimespan > 0.0 {
            self.menu.set_area(96, 0, 32, 35, 0, 0);
            sr.draw(&mview);
        }

        sr.reset_state();

        // Render lod

        let models = self.theModels.as_ref().unwrap().borrow();
        // Render motion triangle
        let motionBlend = models.getMotionBlend();

        // calculate the current motion point
        let motionX = (motionBlend[0] * MENUITEM_MOTION_X[0] as f32
            + motionBlend[1] * MENUITEM_MOTION_X[1] as f32
            + motionBlend[2] * MENUITEM_MOTION_X[2] as f32) as i32;
        let motionY = (motionBlend[0] * MENUITEM_MOTION_Y[0] as f32
            + motionBlend[1] * MENUITEM_MOTION_Y[1] as f32
            + motionBlend[2] * MENUITEM_MOTION_Y[2] as f32) as i32;

        let view = Matrix4::<f32>::identity();
        self.lr.set_state(&view);

        let mview = Matrix4::<f32>::identity();

        self.lr.draw(
            &mview,
            self.menuX + MENUITEM_MOTION_X[0],
            self.menuY + MENUITEM_MOTION_Y[0],
            self.menuX + motionX,
            self.menuY + motionY,
            &[1.0, 1.0, 0.0, 1.0],
        );
        self.lr.draw(
            &mview,
            self.menuX + MENUITEM_MOTION_X[1],
            self.menuY + MENUITEM_MOTION_Y[1],
            self.menuX + motionX,
            self.menuY + motionY,
            &[0.0, 1.0, 1.0, 1.0],
        );
        self.lr.draw(
            &mview,
            self.menuX + MENUITEM_MOTION_X[2],
            self.menuY + MENUITEM_MOTION_Y[2],
            self.menuX + motionX,
            self.menuY + motionY,
            &[1.0, 0.0, 1.0, 1.0],
        );

        self.lr.reset_state();
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

    pub fn key_event(&self, key: Key, action: Action) {}

    pub fn button_event(&self, button: glfw::MouseButton, action: Action, x: i32, y: i32) -> bool {
        false
    }

    pub fn cursor_event(&mut self, x: i32, y: i32) -> bool {
        false
    }

    // ----------------------------------------------------------------------------//
    // Handle window resize event                                                 //
    // ----------------------------------------------------------------------------//
    // 429
    fn onResize(&mut self, width: u32, _height: u32) {
        // adjust menu position
        self.menuX = (width - 132) as i32;

        // adjust lod position
        self.lodX = width / 2 - 128;
    }
}
