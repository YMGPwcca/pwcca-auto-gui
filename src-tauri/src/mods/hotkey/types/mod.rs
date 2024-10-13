#![allow(dead_code)]

pub mod keyboard;

use windows::Win32::UI::Input::KeyboardAndMouse::{HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN};

use keyboard::KeyboardKey;

#[derive(Debug)]
pub struct Key {
  pub id: i32,
  pub name: String,
  pub key: KeyboardKey,
  pub modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone, Copy)]
pub enum Modifier {
  Shift = MOD_SHIFT.0 as isize,
  Ctrl = MOD_CONTROL.0 as isize,
  Alt = MOD_ALT.0 as isize,
  Win = MOD_WIN.0 as isize,
}

impl From<Modifier> for HOT_KEY_MODIFIERS {
  fn from(val: Modifier) -> Self {
    HOT_KEY_MODIFIERS(val as u32)
  }
}

impl From<&Modifier> for HOT_KEY_MODIFIERS {
  fn from(val: &Modifier) -> Self {
    HOT_KEY_MODIFIERS(*val as u32)
  }
}
