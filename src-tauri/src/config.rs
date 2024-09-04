// Mew was here
#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PowerConfig {
  pub timer: u32,
  pub percentage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct MicrophoneConfig {
  pub enabled: bool,
  pub list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
  pub startup: bool,

  pub microphone: MicrophoneConfig,
  pub ethernet: bool,
  pub taskbar: bool,
  pub power: PowerConfig,
}

impl Config {
  pub const fn new() -> Self {
    Config {
      startup: false,
      microphone: MicrophoneConfig {
        enabled: false,
        list: Vec::new(),
      },
      ethernet: false,
      taskbar: false,
      power: PowerConfig {
        timer: 300,
        percentage: 60,
      },
    }
  }

  fn get_path() -> Result<std::path::PathBuf> {
    let exe_path = std::env::current_exe()?;
    let config_path = std::path::Path::new(exe_path.parent().unwrap()).join("config.json");
    Ok(config_path)
  }

  pub fn toggle_startup(&mut self) {
    self.startup = !self.startup;
  }

  pub fn toggle_microphone(&mut self) {
    self.microphone = MicrophoneConfig {
      enabled: !self.microphone.enabled,
      list: self.microphone.list.clone(),
    };
  }

  pub fn toggle_ethernet(&mut self) {
    self.ethernet = !self.ethernet;
  }

  pub fn toggle_taskbar(&mut self) {
    self.taskbar = !self.taskbar;
  }

  pub fn set_power(&mut self, timer: u32, percentage: u32) {
    self.power = PowerConfig { timer, percentage };
  }

  pub fn write(&self) -> Result<Self> {
    std::fs::write(Config::get_path()?, self.stringify()?)?;
    Ok(self.clone())
  }

  pub fn read() -> Result<Self> {
    let path = Config::get_path()?;
    if path.exists() {
      Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    } else {
      Ok(Config::new())
    }
  }

  pub fn stringify(&self) -> Result<String> {
    Ok(serde_json::to_string_pretty(self)?)
  }
}
