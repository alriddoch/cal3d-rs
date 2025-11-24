use super::graphics;
use super::model::{STATE_FANCY, STATE_IDLE, STATE_MOTION};
use super::models::*;
use crate::graphics::RendererError;
use crate::graphics::{LineRenderer, WithOrtho};
use crate::graphics::{Sprite, SpriteError};
use cgmath::Matrix4;
use cgmath::SquareMatrix;
use cgmath::Vector3;
use glfw::{Action, Key};
use std::cell::RefCell;
use std::ops::Mul;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Menu {
    menuX: i32,
    menuY: i32,
    lodX: i32,
    lodY: i32,
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
        Self {
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

    // ----------------------------------------------------------------------------//
    // Get the menu item at a given position                                      //
    // ----------------------------------------------------------------------------//
    fn getMenuItem(&self, x: i32, y: i32) -> usize {
        // check if the point is inside the menu
        if !self.isInside(x, y) {
            return 999;
        }

        // check for the lod bar
        if (x - self.lodX >= 0)
            && (x - self.lodX < 256)
            && (y - self.lodY >= 0)
            && (y - self.lodY < 32)
        {
            return 9;
        }

        // check for each menu item
        for item_id in 0..5 {
            if (y - self.menuY >= MENUITEM_Y[item_id])
                && (y - self.menuY < MENUITEM_Y[item_id] + MENUITEM_HEIGHT[item_id])
            {
                return item_id;
            }
        }

        // test for flag menu items
        if (y - self.menuY >= 0) && (y - self.menuY < 35) {
            return (5 + (x - self.menuX) / 32) as usize;
        }

        return 999;
    }

    // ----------------------------------------------------------------------------//
    // Check if point is inside the menu                                          //
    // ----------------------------------------------------------------------------//
    fn isInside(&self, x: i32, y: i32) -> bool {
        if (x - self.menuX >= 0)
            && (x - self.menuX < 128)
            && (y - self.menuY >= 0)
            && (y - self.menuY < 256)
        {
            return true;
        }
        if (x - self.lodX >= 0)
            && (x - self.lodX < 256)
            && (y - self.lodY >= 0)
            && (y - self.lodY < 32)
        {
            return true;
        }

        return false;
    }

    pub fn onInit(
        &mut self,
        models: Rc<RefCell<Models>>,
        sprite_renderer: Rc<RefCell<graphics::SpriteRenderer>>,
        path: &str,
        width: i32,
        height: i32,
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

        self.lr.Setup(&WithOrtho(width as u32, height as u32))?;

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

    pub fn button_down_event(&mut self, button: glfw::MouseButton, x: i32, y: i32) -> bool {
        // get activated menu item
        let menuItem = self.getMenuItem(x, y);

        if self.theModels.is_none() {
            return false;
        }

        let models = self.theModels.as_ref().unwrap();

        // handle 'idle' button
        if menuItem == STATE_IDLE {
            models.borrow_mut().setState(STATE_IDLE, 0.3);
            return true;
        }

        // handle 'fancy' button
        if menuItem == STATE_FANCY {
            models.borrow_mut().setState(STATE_FANCY, 0.3);
            return true;
        }

        // handle 'motion' button/controller
        if menuItem == STATE_MOTION {
            models.borrow_mut().setState(STATE_MOTION, 0.3);
            self.calculateMotionBlend(x, y);
            self.bMotionMovement = true;
            return true;
        }

        // handle 'f/x 1' button
        if menuItem == 3 {
            models.borrow_mut().executeAction(0);
            self.actionTimespan[0] = 1.0;
        }

        // handle 'f/x 2' button
        if menuItem == 4 {
            models.borrow_mut().executeAction(1);
            self.actionTimespan[1] = 1.0;
        }

        // handle 'skeleton' button
        if menuItem == 5 {
            self.bSkeleton = (self.bSkeleton + 1) % 3;
        }

        // handle 'wireframe' button
        if menuItem == 6 {
            self.bWireframe = !self.bWireframe;
        }

        // handle 'light' button
        if menuItem == 7 {
            self.bLight = !self.bLight
        }

        // handle 'next model' button
        if menuItem == 8 {
            models.borrow_mut().nextModel();
            self.nextTimespan = 0.3
        }

        // handle lod bar
        if menuItem == 9 {
            self.calculateLodLevel(x, y);
            self.bLodMovement = true;
            return true;
        }

        return self.isInside(x, y);
    }

    pub fn button_up_event(&mut self, button: glfw::MouseButton, x: i32, y: i32) -> bool {
        if self.bMotionMovement {
            self.bMotionMovement = false;
            return true;
        }

        if self.bLodMovement {
            self.bLodMovement = false;
            return true;
        }

        return false;
    }

    pub fn cursor_event(&mut self, x: i32, y: i32) -> bool {
        if self.bMotionMovement {
            self.calculateMotionBlend(x, y);

            return true;
        }

        if self.bLodMovement {
            self.calculateLodLevel(x, y);

            return true;
        }
        false
    }

    // ----------------------------------------------------------------------------//
    // Handle window resize event                                                 //
    // ----------------------------------------------------------------------------//
    // 429
    fn onResize(&mut self, width: i32, _height: i32) {
        // adjust menu position
        self.menuX = (width - 132) as i32;

        // adjust lod position
        self.lodX = width / 2 - 128;
    }

    fn calculateLodLevel(&mut self, x: i32, y: i32) {
        // convert to local coordinates
        let x = x - self.lodX;
        let y = y - self.lodY;

        // calculate the new lod level from the local coordinates
        let mut lodLevel = (247 - x) as f32 / 200.0;

        // clamp the value to [0.0, 1.0]
        if lodLevel < 0.0 {
            lodLevel = 0.0
        }
        if lodLevel > 1.0 {
            lodLevel = 1.0
        }

        // set new motion blend factors
        if self.theModels.is_some() {
            match self.theModels.as_ref().unwrap().try_borrow_mut() {
                Ok(mut models) => {
                    models.setLodLevel(lodLevel);
                }
                Err(_) => {
                    println!("Unable to borrow menu.theModels");
                }
            }
        }
    }

    // ----------------------------------------------------------------------------//
    // Calculate new motion blend factors for a given position                    //
    // ----------------------------------------------------------------------------//
    fn calculateMotionBlend(&mut self, x: i32, y: i32) {
        // convert to local coordinates
        let x = x - self.menuX;
        let y = y - self.menuY;

        // check if point is inside motion area
        if (y >= MENUITEM_Y[STATE_MOTION])
            && (y < MENUITEM_Y[STATE_MOTION] + MENUITEM_HEIGHT[STATE_MOTION])
        {
            // calculate baryzentric coordinates inside motion triangle
            let mut motionBlend: [f32; 3] = [0.0, 0.0, 0.0];
            motionBlend[0] = 1.0
                - ((x - MENUITEM_MOTION_X[0]) as f32 + (MENUITEM_MOTION_Y[0] - y) as f32 / 1.732)
                    / 76.0;

            // clamp first to range [0.0 - 1.0]
            if motionBlend[0] < 0.0 {
                motionBlend[0] = 0.0
            }
            if motionBlend[0] > 1.0 {
                motionBlend[0] = 1.0
            }

            motionBlend[1] = 1.0 - (y - MENUITEM_MOTION_Y[1]) as f32 / 66.0;

            // clamp second to range [0.0 - 1.0]
            if motionBlend[1] < 0.0 {
                motionBlend[1] = 0.0
            }
            if motionBlend[1] > 1.0 {
                motionBlend[1] = 1.0
            }

            // clamp sum of first and second to range [0.0 - 1.0]
            if motionBlend[0] + motionBlend[1] > 1.0 {
                let factor = motionBlend[0] + motionBlend[1];
                motionBlend[0] /= factor;
                motionBlend[1] /= factor;
            }

            motionBlend[2] = 1.0 - motionBlend[0] - motionBlend[1];

            // clamp third to range [0.0 - 1.0]
            if motionBlend[2] < 0.0 {
                motionBlend[2] = 0.0
            }

            if self.theModels.is_some() {
                match self.theModels.as_ref().unwrap().try_borrow_mut() {
                    Ok(mut models) => models.setMotionBlend(motionBlend, 0.1),
                    Err(_) => {
                        println!("Unable to borrow menu.theModels");
                    }
                }
                // set new motion blend factors
            }
        }
    }
}
