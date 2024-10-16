// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod mods;
mod panic_catching;
mod threading;

use std::time::{self};

use config::Config;
use mods::{
  media::Media,
  power::get_power_status,
  process::{get_process_executable_name_from_path, get_process_executable_path, get_processes_by_name},
  program::Program,
  startup::task_scheduler::TaskScheduler,
};
use threading::*;

use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  Builder, Emitter, Manager, PhysicalPosition, PhysicalSize, Wry,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use windows::Win32::System::SystemInformation::GetTickCount64;

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
      let handle = app.handle();
      let window = app.get_webview_window("main").unwrap();
      let app_size = PhysicalSize::new(350u32, 750u32);
      window.set_size(app_size)?;
      window.set_always_on_top(true)?;
      window.set_skip_taskbar(true)?;

      // Threading
      build_thread("Power_Thread".to_string(), power_thread);
      build_thread("Media_Thread".to_string(), media_thread);
      build_thread("Connection_Thread".to_string(), connection_thread);
      build_thread("Taskbar_Thread".to_string(), taskbar_thread);
      build_thread("Autostart_Thread".to_string(), autostart_thread);

      // Hotkey specific
      let monitor = window.current_monitor().expect("Cannot get current monitor").unwrap();
      let monitor_size = monitor.size();

      let mute_size = PhysicalSize::new(140, 80);
      let mute_position = PhysicalPosition::new(monitor_size.width - (mute_size.width + 20), 20);
      let mute_view = handle.get_webview_window("mute").unwrap();
      mute_view.set_size(mute_size).expect("Cannot set mute view size");
      mute_view
        .set_position(mute_position)
        .expect("Cannot set mute view position");
      mute_view
        .set_ignore_cursor_events(true)
        .expect("Cannot set ignore cursor events for mute view");
      mute_view.open_devtools();

      let win_f2_shortcut = Shortcut::new(Some(Modifiers::SUPER), Code::F2);
      handle
        .plugin(
          tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |handle, shortcut, event| {
              if shortcut == &win_f2_shortcut && event.state() == ShortcutState::Released {
                #[derive(Clone, serde::Serialize)]
                struct Payload {
                  mute: bool,
                  rgba: Vec<Vec<u8>>,
                }

                let pid = Program::get_foreground_program();
                let path = get_process_executable_path(pid);
                let name = get_process_executable_name_from_path(&path);
                let icon_rgba = Program::get_icon(&path);
                let media = Media::new().expect("Cannot initialize media module");

                if let Some(mute) = media.get_mute_program(&name) {
                  media.set_mute_program(&name, !mute);
                  handle
                    .emit_to(
                      "mute",
                      "program_name",
                      Payload {
                        mute: !mute,
                        rgba: icon_rgba,
                      },
                    )
                    .expect("Cannot emit to mute_window");
                }
              }
            })
            .build(),
        )
        .expect("Cannot build global shortcut");
      app
        .global_shortcut()
        .register(win_f2_shortcut)
        .expect("Cannot register global shortcut");

      Ok(())
    })
    .on_window_event(|window, event| match event {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        window.hide().unwrap();
        api.prevent_close();
      }

      #[cfg(not(debug_assertions))]
      tauri::WindowEvent::Focused(focused) => {
        if !focused && window.label() == "main" {
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
  panic_catching::init();
  panic_catching::trace(|| {
    // Check if another instance is running
    if get_processes_by_name("PwccaAutoGUI")
      .expect("Cannot get processes")
      .len()
      > 1
    {
      panic!("Another instance is already running");
    }

    unsafe {
      if time::Duration::from_millis(GetTickCount64()).as_secs() < 60 && !get_power_status().is_plugged_in {
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
}
