// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mods;

use mods::{
  connection::{is_ethernet_plugged_in, set_wifi_state},
  display::{get_all_frequencies, get_current_frequency, set_new_frequency, turn_off_monitor},
};
use sysinfo::System;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

// #[tauri::command]
// fn get_refresh_rate() -> u32 {
//   let refresh_rate = get_current_frequency();
//   return refresh_rate;
// }

// #[tauri::command]
// fn set_refresh_rate() -> bool {
// let refresh_rate = get_current_frequency();
// let max_refresh_rate = get_all_frequencies().last().copied().unwrap();
// let result = std::panic::catch_unwind(|| set_new_frequency(if refresh_rate == 60 { max_refresh_rate } else { 60 }));

//   if result.is_err() {
//     return false;
//   }
//   return true;
// }

// #[tauri::command]
// fn turn_off_screen() {
//   turn_off_monitor()
// }

fn main() {
  let s = System::new_all();
  let new_all = s.processes_by_name("pwcca-auto");
  for i in new_all {
    if std::process::id() != i.pid().as_u32() {
      std::process::exit(0);
    }
  }

  // Threading
  let _ = std::thread::spawn(move || media_thread());
  let _ = std::thread::spawn(move || connection_thread());

  // Tauri
  tauri::Builder::default()
    .system_tray(
      SystemTray::new().with_tooltip("Pwcca Auto").with_menu(
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new(
            "refresh_rate",
            format!("Refresh Rate: {} Hz", get_current_frequency()),
          ))
          .add_native_item(tauri::SystemTrayMenuItem::Separator)
          .add_item(CustomMenuItem::new("quit", "Quit")),
      ),
    )
    .on_system_tray_event(|app, event| match event {
      // SystemTrayEvent::LeftClick { .. } => {
      //   let window = app.get_window("main").unwrap();
      //   window.show().unwrap();
      //   window.set_focus().unwrap();
      // }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let item_handle = app.tray_handle().get_item(&id);

        match id.as_str() {
          "refresh_rate" => {
            let refresh_rate = get_current_frequency();
            let max_refresh_rate = get_all_frequencies().last().copied().unwrap();
            set_new_frequency(if refresh_rate == 60 { max_refresh_rate } else { 60 });

            item_handle
              .set_title(format!("Refresh Rate: {} Hz", get_current_frequency()))
              .unwrap();
          }
          "quit" => std::process::exit(0),
          _ => {}
        }
      }
      _ => {}
    })
    // .invoke_handler(tauri::generate_handler![
    //   get_refresh_rate,
    //   set_refresh_rate,
    //   turn_off_screen,
    // ])
    // .on_window_event(|event| match event.event() {
    //   tauri::WindowEvent::CloseRequested { api, .. } => {
    //     event.window().hide().unwrap();
    //     api.prevent_close();
    //   }
    //   tauri::WindowEvent::Focused(focused) => {
    //     if focused == &false {
    //       event.window().hide().unwrap()
    //     }
    //   }
    //   _ => {}
    // })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(move |_, _| {});
}

fn media_thread() -> Result<(), mods::media::types::error::AudioDeviceError> {
  // Initialize the media thread
  println!("  + Running Media Thread");

  mods::media::init()?;

  let mut connected = false;
  let discord_executable = String::from("Discord.exe");

  loop {
    // Get all output devices
    let all_outputs = mods::media::enumerate_audio_devices(&mods::media::types::device::DeviceType::Output)?;

    // Check if there are multiple output devices
    if all_outputs.len() > 1 {
      // Get the current default output device
      let current_output = mods::media::get_default_device(&mods::media::types::device::DeviceType::Output)?;

      // Check if Discord is running and recording from default input device
      let programs = mods::media::get_active_audio_applications(&mods::media::types::device::DeviceType::Input)?;

      if programs.contains(&discord_executable) {
        connected = true;

        // Switch to headphones if Discord is recording and speakers are the default
        if current_output.device_type == "Speakers" {
          let headphones = all_outputs
            .iter()
            .find(|device| device.device_type == "Headphones")
            .unwrap();

          mods::media::change_default_output(headphones.device_id)?
        }
      } else if connected {
        connected = false;

        // Switch back to speakers if Discord is not recording and headphones are the default
        if current_output.device_type == "Headphones" {
          let headphones = all_outputs
            .iter()
            .find(|device| device.device_type == "Speakers")
            .unwrap();

          mods::media::change_default_output(headphones.device_id)?
        }
      }
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

fn connection_thread() {
  // Initialize the connection thread
  println!("  + Running Connection Thread");

  loop {
    let _ = set_wifi_state(!is_ethernet_plugged_in());

    // println!("LOG FROM CONNECTION THREAD");
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}
