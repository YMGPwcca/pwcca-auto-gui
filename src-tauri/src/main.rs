// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod mods;
mod threading;
mod types;
mod windows_event;

use std::{process::exit, sync, thread};

use config::Config;
use mods::{
  display::{get_all_frequencies, get_current_frequency, set_new_frequency, turn_off_monitor},
  process::get_processes_by_name,
  startup::task_scheduler::{create_startup_task, delete_startup_task},
};
use threading::*;
use types::Events;

use anyhow::Result;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use windows::{
  core::w,
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL},
  },
};

pub static mut CONFIG: Config = Config::new();

#[tauri::command]
fn get_refresh_rate() -> u32 {
  let refresh_rate = get_current_frequency();
  return refresh_rate;
}

#[tauri::command]
fn set_refresh_rate(app: AppHandle) {
  let refresh_rate = get_current_frequency();
  let max_refresh_rate = get_all_frequencies().last().copied().unwrap();
  set_new_frequency(if refresh_rate == 60 { max_refresh_rate } else { 60 });
  let _ = app.tray_handle().set_menu(build_tray_menu());
}

#[tauri::command]
fn turn_off_screen() {
  turn_off_monitor()
}

pub fn build_tray_menu() -> SystemTrayMenu {
  SystemTrayMenu::new()
    .add_item(CustomMenuItem::new(Events::Startup, "Run with Windows"))
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new(Events::Discord, "Discord"))
    .add_item(CustomMenuItem::new(Events::Ethernet, "Ethernet"))
    .add_item(CustomMenuItem::new(Events::Taskbar, "Taskbar"))
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

  let (sender, receiver) = sync::mpsc::channel::<bool>();

  // Register Windows events
  let w_event = windows_event::WindowsEvent::new(sender);
  w_event.create()?;

  // Tauri
  tauri::Builder::default()
    .any_thread()
    .setup(|app| {
      let handle = app.handle();

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
        .name("Channel_Thread".to_string())
        .spawn(move || channel_thread(handle, receiver));

      Ok(())
    })
    .system_tray(
      SystemTray::new()
        .with_tooltip("Pwcca Auto")
        .with_menu(build_tray_menu()),
    )
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        let window = app.get_window("main").unwrap();

        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          window.set_focus().expect("Cannot focus window");
        }
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let item_handle = app.tray_handle().get_item(&id);

        match id.as_str() {
          "Startup" => {
            unsafe { CONFIG.toggle_startup() };

            if unsafe { CONFIG.startup } {
              create_startup_task().expect("Cannot create startup task");
            } else {
              delete_startup_task().expect("Cannot delete startup task");
            }

            item_handle.set_selected(unsafe { CONFIG.startup }).unwrap();
            unsafe { CONFIG.write().expect("Cannot write config") };
          }
          "Discord" => {
            unsafe { CONFIG.toggle_discord() };

            item_handle.set_selected(unsafe { CONFIG.discord }).unwrap();
            unsafe { CONFIG.write().expect("Cannot write config") };
          }
          "Ethernet" => {
            unsafe { CONFIG.toggle_ethernet() };

            item_handle.set_selected(unsafe { CONFIG.ethernet }).unwrap();
            unsafe { CONFIG.write().expect("Cannot write config") };
          }
          "Taskbar" => {
            unsafe { CONFIG.toggle_taskbar() };

            item_handle.set_selected(unsafe { CONFIG.taskbar }).unwrap();
            unsafe { CONFIG.write().expect("Cannot write config") };
          }
          "TurnOffMonitor" => turn_off_monitor(),
          "RefreshRate" => {
            let refresh_rate = get_current_frequency();
            let max_refresh_rate = get_all_frequencies().last().copied().unwrap();
            set_new_frequency(if refresh_rate == 60 { max_refresh_rate } else { 60 });

            item_handle
              .set_title(format!("Refresh Rate: {} Hz", get_current_frequency()))
              .unwrap();
          }
          "Exit" => app.exit(0),
          _ => {}
        }
      }
      _ => {}
    })
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }

      tauri::WindowEvent::Focused(focused) => {
        if !focused {
          event.window().hide().unwrap();
        }
      }

      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      get_refresh_rate,
      set_refresh_rate,
      turn_off_screen,
    ])
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(move |app, event| match event {
      tauri::RunEvent::Ready => app.get_window("main").unwrap().hide().unwrap(),
      _ => {}
    });

  Ok(())
}
