#![allow(dead_code)]

mod policy_config;
pub mod types;

use types::{Device, DeviceType};

use anyhow::{Error, Result};
use windows::{
  core::{Interface, GUID, PCWSTR, PWSTR},
  Win32::{
    Devices::FunctionDiscovery::{PKEY_DeviceInterface_FriendlyName, PKEY_Device_DeviceDesc},
    Foundation::{BOOL, S_OK},
    Media::Audio::{
      eCapture, eCommunications, eConsole, eRender, AudioSessionStateActive, IAudioSessionControl2,
      IAudioSessionManager2, IMMDevice, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator,
      DEVICE_STATE_ACTIVE,
    },
    System::Com::{CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, STGM_READ},
  },
};

use super::process;

pub struct Media {
  enumerator: IMMDeviceEnumerator,
}
impl Media {
  pub fn new() -> Result<Self> {
    unsafe {
      let res = CoInitialize(None);
      if res.is_err() {
        return Err(Error::msg("Cannot initialize COM"));
      }

      let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

      Ok(Self { enumerator })
    }
  }

  fn get_device_info(&self, device: &IMMDevice) -> Result<Device> {
    unsafe {
      let property_store = device.OpenPropertyStore(STGM_READ).expect("Cannot open property store");
      let device_id = device.GetId().expect("Cannot get device id");
      let device_type = property_store
        .GetValue(&PKEY_Device_DeviceDesc)
        .expect("Cannot get device type")
        .to_string();
      let device_name = property_store
        .GetValue(&PKEY_DeviceInterface_FriendlyName)
        .expect("Cannot get device name")
        .to_string();

      Ok(Device {
        device_object: device.clone(),
        device_id,
        device_type,
        device_name,
      })
    }
  }

  pub fn get_default_device(&self, device_type: &DeviceType) -> Result<Device> {
    unsafe {
      let device = match device_type {
        DeviceType::Input => self
          .enumerator
          .GetDefaultAudioEndpoint(eCapture, eCommunications)
          .expect("Cannot get default input device"),
        DeviceType::Output => self
          .enumerator
          .GetDefaultAudioEndpoint(eRender, eConsole)
          .expect("Cannot get default output device"),
      };

      self.get_device_info(&device)
    }
  }

  pub fn change_default_output(&self, device_id: PWSTR) -> Result<()> {
    unsafe {
      let policy = policy_config::IPolicyConfig::new().expect("Cannot initialize policy configuration");
      policy
        .SetDefaultEndpoint(PCWSTR(device_id.as_ptr()), eConsole)
        .expect("Cannot set default endpoint");

      Ok(())
    }
  }

  pub fn list_all_audio_devices(&self, device_type: &DeviceType) -> Result<Vec<Device>> {
    let mut all_devices = Vec::<Device>::new();

    unsafe {
      let devices = match device_type {
        DeviceType::Input => self
          .enumerator
          .EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE)
          .expect("Cannot enumerate input endpoints"),
        DeviceType::Output => self
          .enumerator
          .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
          .expect("Cannot enumerate output endpoints"),
      };
      for i in 0..devices.GetCount().expect("Cannot get devices count") {
        let device = devices.Item(i).expect("Cannot get audio item");
        all_devices.push(self.get_device_info(&device).expect("Cannot get device info"));
      }

      Ok(all_devices)
    }
  }

  pub fn get_active_audio_programs(&self, device_type: &DeviceType) -> Result<Vec<String>> {
    let mut result = Vec::<String>::new();
    let device = self.get_default_device(device_type).expect("Cannot get default device");

    unsafe {
      let session_manager: IAudioSessionManager2 = device
        .device_object
        .Activate(CLSCTX_ALL, None)
        .expect("Cannot create COM object");
      let session_list = session_manager
        .GetSessionEnumerator()
        .expect("Cannot get audio session enumerator");

      for i in 0..session_list.GetCount().unwrap() {
        let session_control = session_list
          .GetSession(i)
          .expect("Cannot get audio session")
          .cast::<IAudioSessionControl2>()
          .expect("Cannot cast to IAudioSessionControl2");

        if session_control.IsSystemSoundsSession() == S_OK {
          continue;
        }

        let state = session_control.GetState().expect("Cannot get audio session state");
        if state == AudioSessionStateActive {
          let pid = session_control.GetProcessId().expect("Cannot get active program id");
          result.push(process::get_process_executable_name(pid));
        }
      }

      Ok(result)
    }
  }

  pub fn set_mute_program(&self, program: &str, mute: bool) {
    let device = self
      .get_default_device(&DeviceType::Output)
      .expect("Cannot get default output device");

    unsafe {
      let session_manager: IAudioSessionManager2 = device
        .device_object
        .Activate(CLSCTX_ALL, None)
        .expect("Cannot create COM object");
      let session_list = session_manager
        .GetSessionEnumerator()
        .expect("Cannot get audio session enumerator");

      for i in 0..session_list.GetCount().unwrap() {
        let session_control = session_list
          .GetSession(i)
          .expect("Cannot get audio session")
          .cast::<IAudioSessionControl2>()
          .expect("Cannot cast to IAudioSessionControl2");

        if session_control.IsSystemSoundsSession() == S_OK {
          continue;
        }

        let state = session_control.GetState().expect("Cannot get audio session state");
        if state == AudioSessionStateActive {
          let pid = session_control.GetProcessId().expect("Cannot get active program id");
          let name = process::get_process_executable_name(pid);

          if name == program {
            let simple_volume = session_control
              .cast::<ISimpleAudioVolume>()
              .expect("Cannot cast to ISimpleAudioVolume");

            simple_volume
              .SetMute(BOOL(mute.into()), &GUID::zeroed())
              .expect("Cannot mute program");
          }
        }
      }
    }
  }

  pub fn get_mute_program(&self, program: &str) -> Option<bool> {
    let device = self
      .get_default_device(&DeviceType::Output)
      .expect("Cannot get default output device");

    unsafe {
      let session_manager: IAudioSessionManager2 = device
        .device_object
        .Activate(CLSCTX_ALL, None)
        .expect("Cannot create COM object");
      let session_list = session_manager
        .GetSessionEnumerator()
        .expect("Cannot get audio session enumerator");

      for i in 0..session_list.GetCount().unwrap() {
        let session_control = session_list
          .GetSession(i)
          .expect("Cannot get audio session")
          .cast::<IAudioSessionControl2>()
          .expect("Cannot cast to IAudioSessionControl2");

        if session_control.IsSystemSoundsSession() == S_OK {
          continue;
        }

        let state = session_control.GetState().expect("Cannot get audio session state");
        if state == AudioSessionStateActive {
          let pid = session_control.GetProcessId().expect("Cannot get active program id");
          let name = process::get_process_executable_name(pid);

          if name == program {
            let simple_volume = session_control
              .cast::<ISimpleAudioVolume>()
              .expect("Cannot cast to ISimpleAudioVolume");

            return Some(simple_volume.GetMute().expect("Cannot get mute status").as_bool());
          }
        }
      }
    }

    None
  }
}

impl Drop for Media {
  fn drop(&mut self) {
    unsafe { CoUninitialize() };
  }
}
