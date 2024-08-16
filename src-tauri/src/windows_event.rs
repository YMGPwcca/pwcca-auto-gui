use std::{ptr, sync::mpsc::Sender};

use anyhow::Result;
use windows::{
  core::{w, PCWSTR},
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Gdi::HBRUSH,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
      CreateWindowExW, DefWindowProcW, GetWindowLongPtrW, RegisterClassW, SetWindowLongPtrW, CREATESTRUCTW,
      GWL_USERDATA, HCURSOR, HICON, WINDOW_EX_STYLE, WM_CREATE, WM_DISPLAYCHANGE, WM_NCDESTROY, WNDCLASSW,
      WNDCLASS_STYLES, WS_OVERLAPPED,
    },
  },
};

pub struct WindowsEvent {
  hwnd: HWND,
  sender: Sender<bool>,
}

impl WindowsEvent {
  pub fn new(sender: Sender<bool>) -> WindowsEvent {
    let wnd_class = WNDCLASSW {
      style: WNDCLASS_STYLES(0),
      lpfnWndProc: Some(WindowsEvent::wndproc),
      cbClsExtra: 0,
      cbWndExtra: 0,
      hInstance: unsafe { GetModuleHandleW(None).unwrap().into() },
      hIcon: HICON(ptr::null_mut()),
      hCursor: HCURSOR(ptr::null_mut()),
      hbrBackground: HBRUSH(ptr::null_mut()),
      lpszMenuName: PCWSTR(ptr::null()),
      lpszClassName: w!("PwccaAutoTrayIcon"),
    };

    unsafe {
      RegisterClassW(ptr::addr_of!(wnd_class));
    }

    WindowsEvent {
      hwnd: HWND::default(),
      sender,
    }
  }

  pub fn create(&self) -> Result<()> {
    let window = Box::new(WindowsEvent {
      hwnd: HWND::default(),
      sender: self.sender.clone(),
    });
    let pointer = Box::into_raw(window);

    unsafe {
      CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("PwccaAutoTrayIcon"),
        w!("Pwcca Auto Tray Icon"),
        WS_OVERLAPPED,
        0,
        0,
        0,
        0,
        None,
        None,
        GetModuleHandleW(None)?,
        Some(pointer as _),
      )?
    };

    Ok(())
  }

  pub fn subproc(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
      match msg {
        WM_DISPLAYCHANGE => {
          self.sender.send(true).unwrap();
          LRESULT(0)
        }

        _ => DefWindowProcW(self.hwnd, msg, wparam, lparam),
      }
    }
  }

  extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
      match msg {
        WM_CREATE => {
          let create_struct: &mut CREATESTRUCTW = &mut *(lparam.0 as *mut _);
          let window: &mut WindowsEvent = &mut *(create_struct.lpCreateParams as *mut _);
          window.hwnd = hwnd;
          SetWindowLongPtrW(hwnd, GWL_USERDATA, window as *mut _ as _);
          window.subproc(msg, wparam, lparam)
        }

        WM_NCDESTROY => {
          let window_ptr = SetWindowLongPtrW(hwnd, GWL_USERDATA, 0);
          if window_ptr != 0 {
            let window = Box::from_raw(window_ptr as *mut WindowsEvent);
            window.subproc(msg, wparam, lparam)
          } else {
            DefWindowProcW(hwnd, msg, wparam, lparam)
          }
        }

        _ => {
          let window_ptr = GetWindowLongPtrW(hwnd, GWL_USERDATA);
          if window_ptr != 0 {
            let window: &mut WindowsEvent = &mut *(window_ptr as *mut _);
            window.subproc(msg, wparam, lparam)
          } else {
            DefWindowProcW(hwnd, msg, wparam, lparam)
          }
        }
      }
    }
  }
}
