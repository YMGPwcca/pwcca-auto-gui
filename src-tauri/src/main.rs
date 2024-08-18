// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod mods;
mod threading;
mod types;

use std::{process::exit, thread};

use config::Config;
use mods::{
  display::{get_all_frequencies, get_current_frequency, set_new_frequency, turn_off_monitor},
  process::get_processes_by_name,
  startup::task_scheduler::TaskScheduler,
  taskbar::get_taskbar_size,
};
use threading::*;
use types::Events;

use anyhow::Result;
use tauri::{
  CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayEvent, SystemTrayMenu,
  SystemTrayMenuItem,
};
use windows::{
  core::w,
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL},
  },
};

pub static mut CONFIG: Config = Config::new();

pub fn build_tray_menu() -> SystemTrayMenu {
  let mut startup_item = CustomMenuItem::new(Events::Startup, "Run with Windows");
  startup_item.selected = unsafe { CONFIG.startup };
  let mut discord_item = CustomMenuItem::new(Events::Discord, "Discord");
  discord_item.selected = unsafe { CONFIG.discord };
  let mut ethernet_item = CustomMenuItem::new(Events::Ethernet, "Ethernet");
  ethernet_item.selected = unsafe { CONFIG.ethernet };
  let mut taskbar_item = CustomMenuItem::new(Events::Taskbar, "Taskbar");
  taskbar_item.selected = unsafe { CONFIG.taskbar };

  SystemTrayMenu::new()
    .add_item(startup_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(discord_item)
    .add_item(ethernet_item)
    .add_item(taskbar_item)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new(Events::TurnOffMonitor, "Turn off monitor"))
    .add_item(CustomMenuItem::new(
      Events::RefreshRate,
      format!("Refresh Rate: {} Hz", get_current_frequency()),
    ))
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new(Events::Exit, "Quit"))
}

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

  unsafe { CONFIG = Config::read()? }

  // Tauri
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      let monitor = window.current_monitor()?.unwrap();
      let taskbar_size = get_taskbar_size();
      let startup_service_created = TaskScheduler::new()?.is_service_created("PwccaAutoGUI");

      // Sync config with actual settings
      unsafe {
        if CONFIG.startup != startup_service_created {
          CONFIG.toggle_startup();
          CONFIG.write().expect("Cannot write config");
        }
      }

      // Config app size and position
      let monitor_size = monitor.size();
      let app_size = PhysicalSize::new(350u32, 700u32);
      window.set_position(PhysicalPosition::new(
        monitor_size.width - (app_size.width + 20),
        (monitor_size.height - taskbar_size.height) - (app_size.height + 20),
      ))?;
      window.set_size(app_size)?;
      window.set_focus()?;
      window.set_always_on_top(true)?;

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
    .system_tray(
      SystemTray::new()
        .with_tooltip("Pwcca Auto GUI")
        .with_menu(build_tray_menu()),
    )
    .on_system_tray_event(|app, event| {
      let window = app.get_window("main").unwrap();
      let tray_handle = app.tray_handle();

      match event {
        SystemTrayEvent::LeftClick { .. } => {
          if window.is_visible().unwrap() {
            window.hide().unwrap();
          } else {
            window.show().expect("Cannot show window");
            window.set_focus().expect("Cannot focus window");
          }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
          let item = tray_handle.get_item(&id);

          match id.as_str() {
            "Startup" => {
              let task_scheduler = TaskScheduler::new().expect("Cannot construct task scheduler");
              unsafe { CONFIG.toggle_startup() };

              if unsafe { CONFIG.startup } {
                let _ = task_scheduler.create_startup_task("PwccaAutoGUI");
              } else {
                let _ = task_scheduler.delete_startup_task("PwccaAutoGUI");
              }

              item.set_selected(unsafe { CONFIG.startup }).unwrap();
              unsafe { CONFIG.write().expect("Cannot write config") };
            }
            "Discord" => {
              unsafe { CONFIG.toggle_discord() };

              item.set_selected(unsafe { CONFIG.discord }).unwrap();
              unsafe { CONFIG.write().expect("Cannot write config") };
            }
            "Ethernet" => {
              unsafe { CONFIG.toggle_ethernet() };

              item.set_selected(unsafe { CONFIG.ethernet }).unwrap();
              unsafe { CONFIG.write().expect("Cannot write config") };
            }
            "Taskbar" => {
              unsafe { CONFIG.toggle_taskbar() };

              item.set_selected(unsafe { CONFIG.taskbar }).unwrap();
              unsafe { CONFIG.write().expect("Cannot write config") };
            }
            "TurnOffMonitor" => turn_off_monitor(),
            "RefreshRate" => {
              let refresh_rate = get_current_frequency();
              let max_refresh_rate = get_all_frequencies().last().copied().unwrap();
              set_new_frequency(if refresh_rate == 60 { max_refresh_rate } else { 60 });

              item
                .set_title(format!("Refresh Rate: {} Hz", get_current_frequency()))
                .unwrap();
            }
            "Exit" => app.exit(0),
            _ => {}
          };

          window.eval("window.location.reload();").expect("Cannot reload window");
        }
        _ => {}
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
      commands::get_ethernet_state,
      commands::set_ethernet_state,
      commands::get_taskbar_state,
      commands::set_taskbar_state
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
