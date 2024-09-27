use serde_json::{json, Value};

use crate::{
  config::Config,
  mods::{
    display::{get_all_frequencies, get_current_frequency, set_new_frequency, turn_off_monitor},
    startup::{
      registry::{get_all_startup_items, types::startup_status::StartupState},
      task_scheduler::TaskScheduler,
    },
  },
  CONFIG,
};

#[tauri::command]
pub fn get_refresh_rate() -> bool {
  get_current_frequency() != 60
}

#[tauri::command]
pub fn set_refresh_rate() -> Result<(), Value> {
  match get_all_frequencies().last() {
    Some(_) => {
      set_new_frequency(if get_current_frequency() == 60 {
        *get_all_frequencies().last().unwrap()
      } else {
        60
      });
      Ok(())
    }
    None => Err(json!({ "kind": "Refresh rate", "error": "Cannot set refresh rate" })),
  }
}

#[tauri::command]
pub fn turn_off_screen() {
  turn_off_monitor()
}

#[tauri::command]
pub fn get_run_with_windows() -> Result<bool, Value> {
  let task_scheduler = TaskScheduler::new();

  match task_scheduler {
    Ok(task_scheduler) => Ok(task_scheduler.is_service_created("PwccaAutoGUI")),
    Err(e) => Err(json!({ "kind": "Startup", "error": e.to_string() })),
  }
}

#[tauri::command]
pub fn set_run_with_windows() -> Result<(), Value> {
  let task_scheduler = TaskScheduler::new();

  match task_scheduler {
    Ok(task_scheduler) => {
      let service_created = task_scheduler.is_service_created("PwccaAutoGUI");
      if !service_created {
        let _ = task_scheduler.create_startup_task("PwccaAutoGUI");
      } else {
        let _ = task_scheduler.delete_startup_task("PwccaAutoGUI");
      }

      unsafe {
        CONFIG.toggle_startup();
        match CONFIG.write() {
          Ok(_) => Ok(()),
          Err(e) => Err(json!({ "kind": "Config", "error": e.to_string() })),
        }
      }
    }
    Err(e) => Err(json!({ "kind": "Startup", "error": e.to_string() })),
  }
}

#[tauri::command]
pub fn get_config() -> Result<Config, Value> {
  let read = Config::read();

  match read {
    Ok(config) => unsafe {
      CONFIG = config.clone();
      Ok(config)
    },
    Err(e) => Err(json!({ "kind": "Config", "error": e.to_string() })),
  }
}

#[tauri::command]
pub fn save_config(config: String) -> Result<(), Value> {
  unsafe {
    CONFIG = Config::parse(config);
    match CONFIG.write() {
      Ok(_) => Ok(()),
      Err(e) => Err(json!({ "kind": "Config", "error": e.to_string() })),
    }
  }
}

#[tauri::command]
pub fn get_autostart_apps() -> Result<Vec<StartupState>, Value> {
  get_all_startup_items().expect("Cannot get all startup items");

  match get_all_startup_items() {
    Ok(items) => Ok(items),
    Err(e) => Err(json!({ "kind": "Startup", "error": e.to_string() })),
  }
}

#[tauri::command]
pub fn exit_app(app: tauri::AppHandle) {
  app.exit(0)
}
