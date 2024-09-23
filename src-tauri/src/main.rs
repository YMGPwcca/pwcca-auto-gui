// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod mods;
mod threading;

use std::{process::exit, thread};

use config::Config;
use mods::{process::get_processes_by_name, startup::task_scheduler::TaskScheduler, taskbar::get_taskbar_size};
use threading::*;

use anyhow::Result;
use tauri::{Manager, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayEvent};
use windows::{
  core::w,
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL},
  },
};

pub static mut CONFIG: Config = Config::new();

fn main() -> Result<()> {
  // Check if another instance is running
  if get_processes_by_name("PwccaAuto")?.len() > 1 {
    unsafe {
      MessageBoxW(
        HWND::default(),
        w!("Another instance is already running"),
        w!("Error"),
        MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
      )
    };

    exit(1);
  }

  // Read config from file
  unsafe { CONFIG = Config::read()? };

  // Tauri
  tauri::Builder::default()
    .setup(|app| {
      // Sync config with actual settings
      unsafe {
        if CONFIG.startup != TaskScheduler::new()?.is_service_created("PwccaAutoGUI") {
          CONFIG.toggle_startup();
          CONFIG.write().expect("Cannot write config");
        }
      }

      let window = app.get_window("main").unwrap();
      let monitor = window.current_monitor()?.unwrap();
      let monitor_size = monitor.size();
      let app_size = PhysicalSize::new(350u32, 750u32);

      // Config app size and position
      window.set_position(PhysicalPosition::new(
        monitor_size.width - (app_size.width + 20),
        (monitor_size.height - get_taskbar_size().height) - (app_size.height + 20),
      ))?;
      window.set_size(app_size)?;
      window.set_focus()?;
      window.set_always_on_top(true)?;
      window.set_skip_taskbar(true)?;

      // Threading
      let _ = thread::Builder::new()
        .name("Power_Thread".to_string())
        .spawn(power_thread);
      let _ = thread::Builder::new()
        .name("Media_Thread".to_string())
        .spawn(media_thread);
      let _ = thread::Builder::new()
        .name("Connection_Thread".to_string())
        .spawn(connection_thread);
      let _ = thread::Builder::new()
        .name("Taskbar_Thread".to_string())
        .spawn(taskbar_thread);

      Ok(())
    })
    .system_tray(SystemTray::new().with_tooltip("Pwcca Auto GUI"))
    .on_system_tray_event(|app, event| {
      let window = app.get_window("main").unwrap();

      if let SystemTrayEvent::LeftClick { .. } = event {
        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.eval("window.location.reload();").expect("Cannot reload window");
          let app_size = window.outer_size().expect("Cannot get app size");
          let monitor = window.current_monitor().expect("Cannot get monitor").unwrap();
          let monitor_size = monitor.size();

          window
            .set_position(PhysicalPosition::new(
              monitor_size.width - (app_size.width + 20),
              (monitor_size.height - get_taskbar_size().height) - (app_size.height + 20),
            ))
            .expect("Cannot set window position");
          window.show().expect("Cannot show window");
          window.set_focus().expect("Cannot focus window");
        }
      };
    })
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }

      #[cfg(not(debug_assertions))]
      tauri::WindowEvent::Focused(focused) => {
        if !focused {
          event.window().hide().unwrap();
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_refresh_rate,
      commands::set_refresh_rate,
      commands::turn_off_screen,
      commands::get_run_with_windows,
      commands::set_run_with_windows,
      commands::get_config,
      commands::save_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
