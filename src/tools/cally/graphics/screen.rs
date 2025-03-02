use gl;
use glfw::Context;
use glfw::{self};

pub struct Screen {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl Screen {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let Some((mut window, events)) =
            glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        else {
            return Err(String::from("Create window failed"));
        };
        let (screen_width, screen_height) = window.get_framebuffer_size();

        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        // glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

        // unsafe {
        //     gl::Viewport(0, 0, screen_width, screen_height);
        //     gl::ClearColor(0.4, 0.4, 0.4, 1.0);
        // }
        // ---------------

        Ok(Screen {
            glfw: glfw,
            window: window,
            events: events,
        })
    }

    pub fn messages(&mut self) -> glfw::FlushedMessages<'_, (f64, glfw::WindowEvent)> {
        self.glfw.poll_events();
        let messages = glfw::flush_messages(&self.events);
        messages
    }

    pub fn world(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn overlay(&self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }

    pub fn swap(&mut self) {
        unsafe {
            let glerr = gl::GetError();
            if glerr != gl::NO_ERROR {
                println!("Swap OpenGL Error: {glerr}");
            }

            self.window.swap_buffers();

            // glfw::PollEvents();
        }
    }
}
