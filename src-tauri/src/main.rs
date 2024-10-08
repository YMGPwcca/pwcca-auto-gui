// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod mods;
mod threading;

use std::{panic, thread, time::Duration};

use config::Config;
use mods::{power::get_power_status, process::get_processes_by_name, startup::task_scheduler::TaskScheduler};
use threading::*;

use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  Builder, Manager, PhysicalPosition, PhysicalSize, Wry,
};
use windows::{
  core::{w, BSTR},
  Win32::{
    Foundation::HWND,
    System::SystemInformation::GetTickCount64,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL},
  },
};

pub static mut CONFIG: Config = Config::new();
pub static mut IS_START_WITH_BATTERY: bool = false;

fn build_tauri() -> Builder<Wry> {
  Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      // Sync config with actual settings
      unsafe {
        if CONFIG.startup != TaskScheduler::new()?.is_service_created("PwccaAutoGUI") {
          CONFIG.toggle_startup();
          CONFIG.write().expect("Cannot write config");
        }
      }

      let _ = TrayIconBuilder::new()
        .tooltip("PwccaAutoGUI")
        .icon(app.default_window_icon().unwrap().clone())
        .menu_on_left_click(false)
        .menu(&(Menu::with_items(app, &[&(MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?)])?))
        .on_menu_event(|app, event| {
          if event.id == "quit" {
            app.exit(0);
          }
        })
        .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            rect,
            ..
          } = event
          {
            let window = tray.app_handle().get_webview_window("main").expect("Cannot get window");
            let app_size = window.outer_size().expect("Cannot get window size");
            let scale_factor = window.scale_factor().expect("Cannot get window scale factor");

            let icon_pos = rect.position.to_physical::<u32>(scale_factor);
            let app_pos = PhysicalPosition::new(icon_pos.x - (app_size.width / 2), icon_pos.y - (app_size.height + 20));

            // Config app position
            window.set_position(app_pos).expect("Cannot set position");
            window.show().expect("Cannot show window");
            window.set_focus().expect("Cannot focus window");
          }
        })
        .build(app)?;

      // Config app size and settings
      let window = app.get_webview_window("main").unwrap();
      let app_size = PhysicalSize::new(350u32, 750u32);
      window.set_size(app_size)?;
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
      let _ = thread::Builder::new()
        .name("Autostart_Thread".to_string())
        .spawn(autostart_thread);

      Ok(())
    })
    .on_window_event(|window, event| match event {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        window.hide().unwrap();
        api.prevent_close();
      }

      #[cfg(not(debug_assertions))]
      tauri::WindowEvent::Focused(focused) => {
        if !focused {
          window.hide().unwrap();
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
      commands::get_autostart_apps,
      commands::get_config,
      commands::save_config,
      commands::exit_app,
    ])
}

fn main() {
  let catch_panic = panic::catch_unwind(|| {
    // Check if another instance is running
    if get_processes_by_name("PwccaAutoGUI")
      .expect("Cannot get processes")
      .len()
      > 1
    {
      panic!("Another instance is already running");
    }

    unsafe {
      if Duration::from_millis(GetTickCount64()).as_secs() < 60 && !get_power_status().is_plugged_in {
        IS_START_WITH_BATTERY = true;
      }
    }

    // Read config from file
    unsafe { CONFIG = Config::read().expect("Cannot read config") };

    // Tauri
    build_tauri()
      .run(tauri::generate_context!())
      .expect("Error while running tauri application");
  });

  match catch_panic {
    Ok(_) => {}
    Err(e) => unsafe {
      if let Some(panic_msg) = e.downcast_ref::<String>() {
        MessageBoxW(
          HWND::default(),
          &BSTR::from(panic_msg),
          w!("PwccaAutoGUI"),
          MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
        );
      } else if let Some(panic_msg) = e.downcast_ref::<&str>() {
        MessageBoxW(
          HWND::default(),
          &BSTR::from(panic_msg.to_string()),
          w!("PwccaAutoGUI"),
          MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
        );
      } else {
        MessageBoxW(
          HWND::default(),
          w!("Program closed unexpectedly"),
          w!("PwccaAutoGUI"),
          MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
        );
      }
    },
  }
}
