use glfw::ffi::{ glfwGetWin32Window, GLFWwindow };
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{ SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE };

pub fn capture_settings(window_ptr: *mut GLFWwindow) {
    let hwnd = unsafe { HWND(glfwGetWin32Window(window_ptr)) };

    unsafe {
        // Exclude the app window from capture software
        SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE).unwrap();
    }
}
