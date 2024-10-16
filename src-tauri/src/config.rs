// Mew was here
#![allow(dead_code)]

use std::{env, fs, path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct MicrophoneConfig {
  pub enabled: bool,
  pub apps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PowerConfig {
  pub enabled: bool,
  pub timer: u32,
  pub percentage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct AutoStartConfig {
  pub enabled: bool,
  pub apps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskbarConfig {
  pub enabled: bool,
  pub apps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
  // Toggles
  pub startup: bool,
  pub ethernet: bool,

  // Configs
  pub microphone: MicrophoneConfig,
  pub power: PowerConfig,
  pub autostart: AutoStartConfig,
  pub taskbar: TaskbarConfig,
}

impl Config {
  pub const fn new() -> Self {
    Config {
      // Toggles
      startup: false,
      ethernet: false,

      // Configs
      microphone: MicrophoneConfig {
        enabled: false,
        apps: Vec::new(),
      },
      power: PowerConfig {
        enabled: false,
        timer: 300,
        percentage: 60,
      },
      autostart: AutoStartConfig {
        enabled: false,
        apps: Vec::new(),
      },
      taskbar: TaskbarConfig {
        enabled: false,
        apps: Vec::new(),
      },
    }
  }

  fn get_path() -> Result<path::PathBuf> {
    let exe_path = env::current_exe()?;
    let config_path = path::Path::new(exe_path.parent().unwrap()).join("config.json");
    Ok(config_path)
  }

  // Toggles
  pub fn toggle_startup(&mut self) {
    self.startup = !self.startup;
  }

  pub fn toggle_ethernet(&mut self) {
    self.ethernet = !self.ethernet;
  }

  // Configs
  pub fn toggle_microphone(&mut self) {
    self.microphone = MicrophoneConfig {
      enabled: !self.microphone.enabled,
      apps: self.microphone.apps.clone(),
    };
  }

  pub fn toggle_power(&mut self) {
    self.power = PowerConfig {
      enabled: !self.power.enabled,
      timer: self.power.timer,
      percentage: self.power.percentage,
    };
  }

  pub fn toggle_autostart(&mut self) {
    self.autostart = AutoStartConfig {
      enabled: !self.autostart.enabled,
      apps: self.autostart.apps.clone(),
    };
  }

  pub fn toggle_taskbar(&mut self) {
    self.taskbar = TaskbarConfig {
      enabled: !self.taskbar.enabled,
      apps: self.taskbar.apps.clone(),
    };
  }

  pub fn set_power(&mut self, timer: u32, percentage: u32) {
    self.power = PowerConfig {
      enabled: self.power.enabled,
      timer,
      percentage,
    };
  }

  pub fn write(&self) -> Result<Self> {
    fs::write(Config::get_path()?, self.stringify()?)?;
    Ok(self.clone())
  }

  pub fn read() -> Result<Self> {
    let path = Config::get_path()?;
    if path.exists() {
      Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    } else {
      let config = Config::new();
      config.write()?;
      Ok(config)
    }
  }

  pub fn stringify(&self) -> Result<String> {
    Ok(serde_json::to_string_pretty(self)?)
  }

  pub fn parse(config: String) -> Self {
    serde_json::from_str::<Config>(&config).expect("Cannot parse config")
  }
}
