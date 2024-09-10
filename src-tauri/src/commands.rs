use crate::{
  config::{Config, MicrophoneConfig},
  mods::{
    display::{get_all_frequencies, get_current_frequency, set_new_frequency, turn_off_monitor},
    startup::task_scheduler::TaskScheduler,
  },
  CONFIG,
};

#[tauri::command]
pub fn get_refresh_rate() -> bool {
  get_current_frequency() != 60
}

#[tauri::command]
pub fn set_refresh_rate() {
  let refresh_rate = get_current_frequency();
  let max_refresh_rate = get_all_frequencies().last().copied().unwrap();
  set_new_frequency(if refresh_rate == 60 { max_refresh_rate } else { 60 });
}

#[tauri::command]
pub fn turn_off_screen() {
  turn_off_monitor()
}

#[tauri::command]
pub fn get_run_with_windows() -> bool {
  let task_scheduler = TaskScheduler::new().expect("Cannot construct task scheduler");

  task_scheduler.is_service_created("PwccaAutoGUI")
}

#[tauri::command]
pub fn set_run_with_windows() {
  let task_scheduler = TaskScheduler::new().expect("Cannot construct task scheduler");
  let service_created = task_scheduler.is_service_created("PwccaAutoGUI");
  if !service_created {
    let _ = task_scheduler.create_startup_task("PwccaAutoGUI");
  } else {
    let _ = task_scheduler.delete_startup_task("PwccaAutoGUI");
  }

  unsafe {
    CONFIG.toggle_startup();
    CONFIG.write().expect("Cannot write config");
  }
}

#[tauri::command]
pub fn get_ethernet_state() -> bool {
  unsafe { CONFIG.ethernet }
}

#[tauri::command]
pub fn set_ethernet_state() {
  unsafe {
    CONFIG.toggle_ethernet();
    CONFIG.write().expect("Cannot write config");
  }
}

#[tauri::command]
pub fn get_taskbar_state() -> bool {
  unsafe { CONFIG.taskbar.enabled }
}

#[tauri::command]
pub fn set_taskbar_state() {
  unsafe {
    CONFIG.toggle_taskbar();
    CONFIG.write().expect("Cannot write config");
  }
}

#[tauri::command]
pub fn get_microphone_state() -> MicrophoneConfig {
  unsafe { CONFIG.microphone.clone() }
}

#[tauri::command]
pub fn get_config() -> Config {
  unsafe {
    CONFIG = Config::read().expect("Cannot read config");
    CONFIG.clone()
  }
}
