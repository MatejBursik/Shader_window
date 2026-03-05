use glfw::ffi::{ glfwGetCocoaWindow, GLFWwindow };
use objc2::runtime::Object;
use objc2::{ msg_send, sel, sel_impl };
use std::ffi::c_void;

pub fn capture_settings(window_ptr: *mut GLFWwindow) {
    const NSWindowSharingNone: i64 = 0;

    unsafe {
        // Get NSWindow*
        let ns_window: *mut Object = glfwGetCocoaWindow(window_ptr) as *mut Object;

        if !ns_window.is_null() {
            // Exclude the app window from capture software
            let _: () = msg_send![ns_window, setSharingType: NSWindowSharingNone];
        }
    }
}
