use once_cell::sync::Lazy;

use super::demo::*;
use super::sprite::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::{
    default,
    path::{Path, PathBuf},
};

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
    theDemo: Option<Rc<RefCell<Demo>>>,
    // sr:      *SpriteRenderer
    // lr:      *graphics.LineRenderer

     menu: Sprite,
     lod:  Sprite,
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
            // sr: sr,

            menu: Sprite::new(),
            lod: Sprite::new(),
        }
    }

    pub fn onInit(&mut self, demo: Rc<RefCell<Demo>>, width: i32, height: i32) {
        self.theDemo = Some(demo);


	// load the menu texture
	let strFilename = [demo.borrow().strDatapath.as_str(), "menu.raw"]
            .iter()
            .collect::<PathBuf>();
    // filepath.Join(demo.borrow().strDatapath, "menu.raw")


	self.menu.WithSpriteFile(strFilename).Setup()?;
	self.sr.Bind(self.menu)

	// load the lodxture
	strFilename = filepath.Join(demo.strDatapath, "lod.raw")

	if err := self.lod.Setup(WithSpriteFile(strFilename)); err != nil {
		return err
	}
	self.sr.Bind(self.lod)

	self.lr = graphics.NewLineRenderer()
	if err := self.lr.Setup(graphics.WithOrtho(width, height)); err != nil {
		return errors.Wrapf(err, "LineRenderer setup failed")
	}

	self.onResize(width, height)
	return nil
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
}
