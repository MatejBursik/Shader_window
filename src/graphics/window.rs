use glfw::{Action, Context, Key, WindowEvent};
use std::collections::HashSet;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,
    windowed_pos: (i32, i32),
    windowed_size: (i32, i32),
    fullscreen: bool,
    overlay_mode: bool
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init_no_callbacks().unwrap();

        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).expect("Failed to create window!");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        let (w, h) = window.get_size();

        Window {glfw, window_handle: window, events, keys_pressed: HashSet::new(), keys_released: HashSet::new(), windowed_pos: (0, 0), windowed_size: (w, h), fullscreen: false, overlay_mode: false}
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();

        gl::load_with(|s| {
            match self.window_handle.get_proc_address(s) {
                    Some(p) => p as *const _,
                    None => std::ptr::null()
                }
        });
    }

    pub fn close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        self.keys_released.clear();
        
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                
                glfw::WindowEvent::Key(key, _, action, _) => match action {
                    Action::Press => {
                        self.keys_pressed.insert(key);

                        if key == Key::Escape {
                            self.window_handle.set_should_close(true);
                        }
                    },

                    Action::Release => {
                        self.keys_pressed.remove(&key);
                        self.keys_released.insert(key);
                    },

                    _ => {}
                },

                _ => {}
            }
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn set_fps(&mut self, interval: i32) {
        self.window_handle.make_current();
        match interval {
            -1 => self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive),
            0 => self.glfw.set_swap_interval(glfw::SwapInterval::None),
            1 => self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1)),
            n if n > 1 => {
                // Many GLFW implementations only support intervals of 0, 1, or adaptive vsync
                // But try to set the specific interval if possible
                // The swap interval mechanism (glfw.set_swap_interval()) only allows you to synchronize
                //  with fractions or multiples of the monitor's refresh rate.
                // Input of 2 or 3 means Sync to every 2nd or 3rd frame (30 FPS or 20 FPS on 60Hz)
                self.glfw.set_swap_interval(glfw::SwapInterval::Sync(n as u32))
            },
            _ => {
                self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1))
            }
        }
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        self.windowed_size
    }

    pub fn set_window_size(&mut self, size: (i32, i32)) {
        self.window_handle.set_size(size.0, size.1);
        self.windowed_size = (size.0, size.1);
    }

    pub fn get_window_pos(&self) -> (i32, i32) {
        self.windowed_pos
    }

    pub fn set_window_pos(&mut self, pos: (i32, i32)) {
        self.window_handle.set_pos(pos.0, pos.1);
        self.windowed_pos = (pos.0 as i32, pos.1 as i32);
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen == self.fullscreen {
            return;
        }

        if fullscreen {
            // save windowed state
            self.windowed_pos = self.window_handle.get_pos();
            self.windowed_size = self.window_handle.get_size();

            self.glfw.with_primary_monitor(|_, m| {
                let monitor = m.expect("No primary monitor");
                let mode = monitor.get_video_mode().unwrap();

                self.window_handle.set_monitor(glfw::WindowMode::FullScreen(&monitor), 0, 0, mode.width, mode.height, Some(mode.refresh_rate));
            });
        } else {
            let (x, y) = self.windowed_pos;
            let (w, h) = self.windowed_size;

            self.window_handle.set_monitor(glfw::WindowMode::Windowed, x, y, w as u32, h as u32, None);
        }

        self.fullscreen = fullscreen;
    }

    pub fn set_borderless_fullscreen(&mut self, enable: bool) {
        if enable {
            // Save windowed state
            self.windowed_pos = self.window_handle.get_pos();
            self.windowed_size = self.window_handle.get_size();

            if self.fullscreen {
                self.set_fullscreen(false);
            }

            self.glfw.with_primary_monitor(|_, m| {
                let monitor = m.expect("No primary monitor");
                let mode = monitor.get_video_mode().unwrap();

                self.window_handle.set_decorated(!enable);
                self.window_handle.set_pos(0, 0);
                self.window_handle.set_size(mode.width as i32, mode.height as i32);
            });
        } else {
            // Restore
            self.window_handle.set_decorated(!enable);

            let (x, y) = self.windowed_pos;
            let (w, h) = self.windowed_size;

            self.window_handle.set_pos(x, y);
            self.window_handle.set_size(w, h);
        }
    }

    pub fn set_overlay_mode(&mut self, enable: bool) {
        self.window_handle.set_floating(enable);
        self.window_handle.set_mouse_passthrough(enable);
        self.set_borderless_fullscreen(enable);
    }

    pub fn toggle_overlay_mode(&mut self) {
        self.overlay_mode = !self.overlay_mode;
        self.set_overlay_mode(self.overlay_mode);
    }
}
