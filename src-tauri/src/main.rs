#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


mod util;
mod command_util;
mod monitor;

use tauri::Manager;
use crate::util::switch_dev_tool;
use crate::monitor::{system_info,process_info};

fn main() {
  tauri::Builder::default()
      .setup(|app|{
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
          let window = app.get_window("main").unwrap();
            switch_dev_tool(&window,true);
        }
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
          system_info,
          process_info
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
