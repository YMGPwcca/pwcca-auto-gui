use std::fmt::{self};

use windows::{core::PWSTR, Win32::Media::Audio::IMMDevice};

#[derive(Debug)]
pub struct Device {
  pub device_object: IMMDevice,
  pub device_id: PWSTR,
  pub device_type: String,
  pub device_name: String,
}
impl fmt::Display for Device {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Name: {}\t Type: {}", self.device_name, self.device_type)
  }
}

#[derive(Clone, Debug)]
pub enum DeviceType {
  Input,
  Output,
}
