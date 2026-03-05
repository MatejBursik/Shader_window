use glfw::ffi::GLFWwindow;

pub fn capture_settings(window_ptr: *mut GLFWwindow) {
    // Fix: reason in todo!
    todo!("Linux capture_settings (Well, neither Wayland nor X11 wanted to cooperate and exclude the application window from being captured. So, it will be implemented later)");
}
