#![allow(dead_code)]

pub mod types;

use types::{keyboard::KeyboardKey, Key, Modifier};
use windows::Win32::UI::{
  Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_NOREPEAT},
  WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

pub struct HotKey {
  keys: Vec<Key>,
}
impl HotKey {
  pub fn new() -> Self {
    HotKey { keys: vec![] }
  }

  pub fn add_key(&mut self, id: i32, name: String, key: KeyboardKey, modifiers: Vec<Modifier>) {
    let mut win_modifiers = MOD_NOREPEAT;

    for modifier in modifiers.iter() {
      win_modifiers |= modifier.into();
    }

    unsafe { RegisterHotKey(None, id, win_modifiers, key as u32).expect("Cannot register global hotkey") };

    self.keys.push(Key {
      id,
      name,
      key,
      modifiers,
    });
  }

  pub fn listen_for_keys<F: Fn(&Key)>(&self, callback: F) {
    loop {
      let mut message = MSG::default();
      if unsafe { GetMessageW(&mut message, None, WM_HOTKEY, WM_HOTKEY).as_bool() } {
        let mut iter_keys = self.keys.iter();
        if let Some(key) = iter_keys.find(|e| e.id == message.wParam.0 as i32) {
          callback(key);
        }
      }
    }
  }
}

impl Drop for HotKey {
  fn drop(&mut self) {
    for key in self.keys.iter() {
      unsafe { UnregisterHotKey(None, key.id).expect("Cannot unregister global hotkey") };
    }
  }
}
