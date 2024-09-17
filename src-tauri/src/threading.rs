use std::{thread, time::Duration};

use crate::mods::{
  connection::{is_ethernet_plugged_in, set_wifi_state},
  media::{
    change_default_output, enumerate_audio_devices, get_active_audio_applications, get_default_device, init,
    types::{device::DeviceType, error::AudioDeviceError},
  },
  power::{get_active_power_scheme, get_all_power_schemes, get_power_status, set_active_power_scheme},
  taskbar::taskbar_automation,
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
      let config_includes = unsafe { &CONFIG.microphone.include };

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
  let power = unsafe { CONFIG.power };

  let powersaver = all_power_schemes
    .iter()
    .find(|scheme| scheme.name == "POWERSAVER")
    .unwrap();
  let ultra = all_power_schemes.iter().find(|scheme| scheme.name == "Ultra").unwrap();

  loop {
    if unsafe { CONFIG.power.enabled } {
      let is_plugged_in = get_power_status().is_plugged_in;

      if on_battery_secs > power.timer || get_power_status().remaining_percentage < power.percentage {
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
