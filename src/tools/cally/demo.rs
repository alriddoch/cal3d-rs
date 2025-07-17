use cgmath::Matrix4;
use cgmath::SquareMatrix;
use cgmath::Vector3;
use cgmath::{Deg, Rad};
use clap::Parser;
use glfw::{Action, Key};
use std::cell::RefCell;
use std::ops::Mul;
use std::path::PathBuf;
use std::rc::Rc;

use super::graphics;
use super::menu::*;
use super::model::*;
use super::models::*;
use super::tick::*;
use crate::graphics::{Sprite, SpriteError};

#[derive(Debug)]
pub enum DemoError {
    MenuError(MenuError),
    ModelError(ModelError),
    SpriteError(SpriteError),
    PathError,
    OtherError(String),
}

impl From<MenuError> for DemoError {
    fn from(error: MenuError) -> Self {
        DemoError::MenuError(error)
    }
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

pub struct DemoControls {
    done: bool,
    mouseX: i32,
    mouseY: i32,
    bPaused: bool,
    bLeftMouseButtonDown: bool,
    bRightMouseButtonDown: bool,
    tiltAngle: f32,
    twistAngle: f32,
    distance: f32,
}

// #[derive(Default)]
pub struct Demo {
    width: u32,
    height: u32,
    bFullscreen: bool,
    fpsDuration: f32,
    fpsFrames: i32,
    fps: i32,
    lastTick: u128,
    pub strDatapath: String,
    strCal3D_Datapath: String,
    averageCPUTime: f32,
    bOutputAverageCPUTimeAtExit: bool,

    start: f64,
    firstTime: f64,
    lastTime: f64,
    bFirst: bool,
    cumul: f64,

    controls: RefCell<DemoControls>,
    theModels: Rc<RefCell<Models>>,
    theMenu: Rc<RefCell<Menu>>,
    screen: RefCell<graphics::Screen>,
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
            strDatapath: String::from("data/"),
            screen: RefCell::new(graphics::Screen::new("foo", 800, 600)?),
            lastTick: 0,
            strCal3D_Datapath: String::from(""),
            averageCPUTime: 0.0,
            bOutputAverageCPUTimeAtExit: false,

            start: 0.0,
            firstTime: 0.0,
            lastTime: 0.0,
            bFirst: true,
            cumul: 0.0,
            controls: RefCell::new(DemoControls {
                done: false,
                mouseX: 0,
                mouseY: 0,
                bPaused: false,

                bLeftMouseButtonDown: false,
                bRightMouseButtonDown: false,
                tiltAngle: -70.0,
                twistAngle: -45.0,
                distance: 270.0,
            }),
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
        )?;

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
            self.handle_events();
            self.onIdle();

            self.onRender();
            self.onRenderInterface();
            self.screen.borrow_mut().swap();

            if self.controls.borrow().done {
                break;
            }
        }
    }

    fn handle_events(&self) {
        use glfw::WindowEvent;

        let mut screen = self.screen.borrow_mut();

        let messages = screen.messages();

        for (_, event) in messages {
            // done = done || self.keys.glfw_handle_event(event);
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    println!("key: {event:?}");
                    self.key_event(key, action);
                }
                WindowEvent::MouseButton(button, action, _) => {
                    println!("mouse_button: {event:?}");
                    self.button_event(button, action);
                }
                WindowEvent::CursorPos(x, y) => {
                    println!("cursor_pos: {event:?}");
                    self.cursor_event(x, y);
                }
                WindowEvent::Size(width, height) => {
                    println!("size: {event:?}");
                    self.size_event(width, height);
                }
                WindowEvent::Close => {
                    println!("close: {event:?}");
                    self.controls.borrow_mut().done = true;
                }
                _ => {
                    println!("other: {event:?}");
                }
            }
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
        if !self.controls.borrow().bPaused {
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

    fn key_event(&self, key: Key, action: Action) {
        match key {
            glfw::Key::Escape => {
                if action == glfw::Action::Release {
                    self.controls.borrow_mut().done = true;
                }
            }
            glfw::Key::Space => {
                if action == glfw::Action::Release {
                    let mut controls = self.controls.borrow_mut();
                    controls.bPaused = !controls.bPaused;
                }
            }
            _ => {}
        }
        self.theMenu.borrow_mut().key_event(key, action);
    }

    fn button_event(&self, button: glfw::MouseButton, action: Action) {
        let mut controls = self.controls.borrow_mut();
        match action {
            glfw::Action::Press => {
                if !self.theMenu.borrow_mut().button_event(
                    button,
                    action,
                    controls.mouseX,
                    controls.mouseY,
                ) {
                    match button {
                        glfw::MouseButtonLeft => {
                            controls.bLeftMouseButtonDown = true;
                        }
                        glfw::MouseButtonRight => {
                            controls.bRightMouseButtonDown = true;
                        }
                        _ => {}
                    }
                }
            }
            glfw::Action::Release => {}
            _ => {}
        }
    }

    fn cursor_event(&self, x: f64, y: f64) {
        let mut controls = self.controls.borrow_mut();
        let x = x as i32;
        let y = y as i32;

        if !self.theMenu.borrow_mut().cursor_event(x, y) {
            // update twist/tilt angles
            if controls.bLeftMouseButtonDown {
                // calculate new angles
                controls.twistAngle += (x - controls.mouseX) as f32;
                controls.tiltAngle -= (y - controls.mouseY) as f32;
            }

            // update distance
            if controls.bRightMouseButtonDown {
                // calculate new distance
                controls.distance -= (y - controls.mouseY) as f32 / 3.0;
                if controls.distance < 0.0 {
                    controls.distance = 0.0
                }
            }
            // unimplemented!();
        }
        controls.mouseX = x as i32;
        controls.mouseY = y as i32;
    }

    fn size_event(&self, width: i32, height: i32) {}

    fn onRender(&self) {
        self.screen.borrow().world();

        let render_scale = self.theModels.borrow().render_scale();

        let view = Matrix4::<f32>::identity()
            .mul(Matrix4::from_translation(Vector3 {
                x: 0.0,
                y: 0.0,
                z: -self.controls.borrow().distance * render_scale,
            }))
            .mul(Matrix4::from_axis_angle(
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Rad::from(Deg(self.controls.borrow().tiltAngle)),
            )) // view.Rotate(-65, 1, 0, 0)
            .mul(Matrix4::from_axis_angle(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Rad::from(Deg(self.controls.borrow().twistAngle)),
            ))
            .mul(Matrix4::from_translation(Vector3 {
                x: 0.0,
                y: 0.0,
                z: -90.0 * render_scale,
            }));

        // self.theModels.borrow().render(&view);
    }

    fn onRenderInterface(&self) {
        self.screen.borrow().overlay();

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        self.theMenu.borrow().onRender();

        // TODO: more stuff

        // unimplemented!();
    }

    fn setDimension(&self, width: u32, height: u32) {
        unimplemented!();
        // // store new width and height values
        // self.width = width;
        // self.height = height;

        // self.screen.Window.SetSize(width, height);
        // // set new viewport dimension
        // //glViewport(0, 0, self.width, self.height)

        // // adjust menu
        // self.theMenu.onResize(width, height);
    }
}
