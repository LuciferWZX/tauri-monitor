use tauri::{Window, Wry};

pub fn switch_dev_tool(window:&Window<Wry>,open:bool) {
    if open {
        window.open_devtools()
    } else {
        window.close_devtools()
    }
}