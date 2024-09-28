// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod mods;
mod threading;

use std::{panic::catch_unwind, process::exit, thread};

use config::Config;
use mods::{process::get_processes_by_name, startup::task_scheduler::TaskScheduler, taskbar::get_taskbar_size};
use threading::*;

use anyhow::Error;
use tauri::{Manager, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayEvent};
use windows::{
  core::{w, BSTR},
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL},
  },
};

pub static mut CONFIG: Config = Config::new();

fn main() {
  let catch_panic = catch_unwind::<_, Result<_, Error>>(|| {
    // Check if another instance is running
    if get_processes_by_name("PwccaAutoGUI")?.len() > 1 {
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

        #[cfg(debug_assertions)]
        window.open_devtools();

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
        let _ = thread::Builder::new()
          .name("Autostart_Thread".to_string())
          .spawn(autostart_thread);

        Ok(())
      })
      .system_tray(SystemTray::new().with_tooltip("Pwcca Auto GUI"))
      .on_system_tray_event(|app, event| {
        let window = app.get_window("main").unwrap();

        if let SystemTrayEvent::LeftClick { .. } = event {
          let visible = window.is_visible().unwrap();

          if !visible {
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
          } else {
            window.hide().unwrap();
          }
        };
      })
      .on_window_event(|event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
          event.window().hide().unwrap();
          api.prevent_close();
        }
      })
      .invoke_handler(tauri::generate_handler![
        commands::get_refresh_rate,
        commands::set_refresh_rate,
        commands::turn_off_screen,
        commands::get_run_with_windows,
        commands::set_run_with_windows,
        commands::get_autostart_apps,
        commands::get_config,
        commands::save_config,
        commands::exit_app,
      ])
      .run(tauri::generate_context!())
      .expect("Error while running tauri application");

    Ok(())
  });

  match catch_panic {
    Ok(_) => {}
    Err(e) => unsafe {
      if let Some(panic_msg) = e.downcast_ref::<String>() {
        MessageBoxW(
          HWND::default(),
          &BSTR::from(panic_msg),
          w!("Error"),
          MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
        );
      } else if let Some(panic_msg) = e.downcast_ref::<&str>() {
        MessageBoxW(
          HWND::default(),
          &BSTR::from(panic_msg.to_string()),
          w!("Error"),
          MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
        );
      } else {
        MessageBoxW(
          HWND::default(),
          w!("Program closed unexpectedly"),
          w!("Error"),
          MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
        );
      }
    },
  }
}
