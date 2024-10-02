use std::{os::windows::process::CommandExt, process::Command, thread, time::Duration};

use crate::{
  mods::{
    connection::{is_ethernet_plugged_in, set_wifi_state},
    media::{
      change_default_output, enumerate_audio_devices, get_active_audio_applications, get_default_device, init,
      types::{device::DeviceType, error::AudioDeviceError},
    },
    power::{get_active_power_scheme, get_all_power_schemes, get_power_status, set_active_power_scheme},
    startup::registry::{get_all_startup_items, get_startup_item_value, set_startup_item_state},
    taskbar::taskbar_automation,
  },
  IS_START_WITH_BATTERY,
};

use anyhow::Result;
use windows::Win32::Foundation::WIN32_ERROR;

use crate::CONFIG;

pub fn media_thread() -> Result<(), AudioDeviceError> {
  // Initialize the media thread
  println!("  + Running Media Thread");

  init()?;

  let mut connected = false;

  loop {
    if unsafe { CONFIG.microphone.enabled } {
      let config_includes = unsafe { &CONFIG.microphone.apps };

      let all_outputs = enumerate_audio_devices(&DeviceType::Output)?;

      if all_outputs.len() > 1 {
        let current_output = get_default_device(&DeviceType::Output)?;

        let programs = get_active_audio_applications(&DeviceType::Input)?;

        if config_includes.iter().any(|e| programs.contains(e)) {
          connected = true;

          if current_output.device_type == "Speakers" {
            let headphones = all_outputs
              .iter()
              .find(|device| device.device_type == "Headphones")
              .unwrap();

            change_default_output(headphones.device_id)?
          }
        } else if connected {
          connected = false;

          if current_output.device_type == "Headphones" {
            let headphones = all_outputs
              .iter()
              .find(|device| device.device_type == "Speakers")
              .unwrap();

            change_default_output(headphones.device_id)?
          }
        }
      }
    }

    thread::sleep(Duration::from_secs(1));
  }
}

pub fn connection_thread() -> Result<()> {
  // Initialize the connection thread
  println!("  + Running Connection Thread");

  loop {
    if unsafe { CONFIG.ethernet } {
      let _ = set_wifi_state(!is_ethernet_plugged_in());
    }

    std::thread::sleep(Duration::from_secs(1));
  }
}

pub fn power_thread() -> Result<(), WIN32_ERROR> {
  // Initialize the power thread
  println!("  + Running Power Thread");

  let mut on_battery_secs = 0;
  let all_power_schemes = get_all_power_schemes()?;
  let power = unsafe { &CONFIG.power };

  let powersaver = all_power_schemes
    .iter()
    .find(|scheme| scheme.name == "POWERSAVER")
    .unwrap();
  let ultra = all_power_schemes.iter().find(|scheme| scheme.name == "Ultra").unwrap();

  loop {
    if power.enabled {
      let is_plugged_in = get_power_status().is_plugged_in;

      if power.timer != 0 && on_battery_secs > power.timer {
        set_active_power_scheme(&powersaver.guid)?;
      }

      if power.percentage != 0 && !is_plugged_in && get_power_status().remaining_percentage < power.percentage {
        set_active_power_scheme(&powersaver.guid)?;
      }

      if is_plugged_in && get_active_power_scheme()?.guid == powersaver.guid {
        set_active_power_scheme(&ultra.guid)?;
      }

      if !is_plugged_in {
        on_battery_secs += 1;
      } else {
        on_battery_secs = 0;
      }
    }

    std::thread::sleep(Duration::from_secs(1));
  }
}

pub fn taskbar_thread() {
  // Initialize the taskbar thread
  println!("  + Running Taskbar Thread");

  loop {
    if unsafe { CONFIG.taskbar.enabled } {
      taskbar_automation();
    }

    std::thread::sleep(Duration::from_secs(1));
  }
}

pub fn autostart_thread() {
  // Initialize the autostart thread
  println!("  + Running Autostart Thread");

  let mut ran = false;

  loop {
    if unsafe { CONFIG.autostart.enabled } {
      let disallow: Vec<String> = unsafe { CONFIG.autostart.apps.clone() };

      let is_plugged_in = get_power_status().is_plugged_in;
      let startup_items = get_all_startup_items().expect("Cannot get all startup items");

      for item in &startup_items {
        if disallow.contains(&item.name) {
          set_startup_item_state(item, is_plugged_in)
            .unwrap_or_else(|_| panic!("Cannot disable {} startup", item.name));
        }
      }

      if unsafe { IS_START_WITH_BATTERY } && is_plugged_in && !ran {
        let _ = thread::Builder::new()
          .name("AutoStart Run Thread".to_string())
          .spawn(move || {
            let list = startup_items
              .iter()
              .filter(|e| disallow.contains(&e.name))
              .collect::<Vec<_>>();

            for item in list {
              let startup_value = get_startup_item_value(item).expect("Cannot get startup item value");
              let mut cmd = Command::new("cmd")
                .raw_arg("/C")
                .raw_arg("start")
                .raw_arg("\"\"")
                .raw_arg(&startup_value)
                .spawn()
                .expect("Cannot start program");

              std::thread::sleep(Duration::from_secs(1));
              cmd.kill().expect("Cannot kill program");
            }
          });

        ran = true;
      }
    }

    std::thread::sleep(Duration::from_secs(1));
  }
}

#[test]
fn test() {
  let status = Command::new("cmd")
    .args([
      "/C",
      "start",
      "/b",
      "",
      "E:\\Riot Games\\Riot Client\\RiotClientServices.exe",
      "--launch-background-mode",
    ])
    .status()
    .expect("Failed to spawn child process");

  // Handle the status of the child process as needed
  println!("Child process exited with status: {}", status.code().unwrap_or(-1));
}
