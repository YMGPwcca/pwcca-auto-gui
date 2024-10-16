#![allow(dead_code)]

use std::{
  mem::{self},
  ptr::{self},
};

use anyhow::{Error, Result};
use windows::{
  core::{Interface, BSTR, VARIANT},
  Win32::{
    Foundation::HWND,
    Graphics::Gdi::{DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFOHEADER, DIB_RGB_COLORS},
    Storage::FileSystem::SECURITY_ANONYMOUS,
    System::Com::{
      CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED,
      COINIT_DISABLE_OLE1DDE,
    },
    UI::{
      Controls::{IImageList, ILD_TRANSPARENT},
      Shell::{
        IShellBrowser, IShellDispatch2, IShellFolderViewDual, IShellView, IShellWindows, IUnknown_QueryService, SHGetFileInfoW, SHGetImageList, SID_STopLevelBrowser, ShellWindows, SHFILEINFOW, SHGFI_SYSICONINDEX, SHIL_EXTRALARGE, SHIL_JUMBO, SVGIO_BACKGROUND, SWC_DESKTOP, SWFO_NEEDDISPATCH
      },
      WindowsAndMessaging::{DestroyIcon, GetForegroundWindow, GetIconInfo, GetWindowThreadProcessId},
    },
  },
};

pub struct Program {
  shell_view: IShellView,
  shell_dispatch: IShellDispatch2,
}

impl Program {
  pub fn new() -> Result<Self> {
    let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) };
    if hr.is_err() {
      return Err(Error::msg(hr.message()));
    }

    let shell_view = Program::get_shell_view()?;
    let shell_dispatch = Program::get_shell_dispatch(&shell_view)?;

    Ok(Program {
      shell_view,
      shell_dispatch,
    })
  }

  fn get_shell_view() -> Result<IShellView> {
    unsafe {
      let shell_windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?;

      let mut hwnd = HWND::default();
      let dispatch = shell_windows.FindWindowSW(
        &VARIANT::default(),
        &VARIANT::default(),
        SWC_DESKTOP,
        std::ptr::addr_of_mut!(hwnd) as _,
        SWFO_NEEDDISPATCH,
      )?;

      let shell_browser: IShellBrowser = IUnknown_QueryService(&dispatch, &SID_STopLevelBrowser)?;
      let shell_view: IShellView = shell_browser.QueryActiveShellView()?;

      Ok(shell_view)
    }
  }

  fn get_shell_dispatch(shell_view: &IShellView) -> Result<IShellDispatch2> {
    unsafe {
      let dispatch_background: IDispatch = shell_view.GetItemObject(SVGIO_BACKGROUND)?;

      let mut p_shell_folder_view_dual = std::ptr::null_mut();
      let hr = dispatch_background.query(&IShellFolderViewDual::IID, &mut p_shell_folder_view_dual);

      if hr.is_err() {
        return Err(Error::msg(hr.message()));
      }

      let shell_folder_view_dual = IShellFolderViewDual::from_raw(p_shell_folder_view_dual);

      let dispatch = shell_folder_view_dual.Application()?;
      let mut p_shell_dispatch_2 = std::ptr::null_mut();
      let hr = dispatch.query(&IShellDispatch2::IID, &mut p_shell_dispatch_2);
      if hr.is_err() {
        return Err(Error::msg(hr.message()));
      }

      Ok(IShellDispatch2::from_raw(p_shell_dispatch_2))
    }
  }

  pub fn run(&self, file: String, args: Option<String>) -> Result<()> {
    unsafe {
      let file = BSTR::from(file);
      let args = if let Some(args) = args {
        VARIANT::from(BSTR::from(args))
      } else {
        VARIANT::default()
      };

      self.shell_dispatch.ShellExecute(
        &file,
        &args,
        &VARIANT::default(),
        &VARIANT::default(),
        &VARIANT::default(),
      )?;
    }

    Ok(())
  }

  pub fn get_foreground_program() -> u32 {
    unsafe {
      let foreground = GetForegroundWindow();

      let mut pid = 0;
      GetWindowThreadProcessId(foreground, Some(&mut pid));
      pid
    }
  }

  pub fn get_icon(path: &str) -> Vec<Vec<u8>> {
    unsafe {
      let path = &BSTR::from(path);

      let mut sfi = SHFILEINFOW::default();
      SHGetFileInfoW(
        path,
        SECURITY_ANONYMOUS,
        Some(&mut sfi),
        mem::size_of_val(&sfi) as u32,
        SHGFI_SYSICONINDEX,
      );

      let ret = SHGetImageList::<IImageList>(SHIL_EXTRALARGE as i32).expect("Cannot get image list");
      let hicon = ret.GetIcon(sfi.iIcon, ILD_TRANSPARENT.0).expect("Cannot get icon");

      let bitmap_size = mem::size_of::<BITMAP>() as i32;
      let biheader_size = mem::size_of::<BITMAPINFOHEADER>() as u32;

      let mut info = mem::MaybeUninit::uninit();
      GetIconInfo(hicon, info.as_mut_ptr()).expect("Cannot get icon info");
      let info = info.assume_init_ref();
      DeleteObject(info.hbmMask).expect("Cannot delete object");
      DestroyIcon(hicon).expect("Cannot destroy icon");

      let mut bitmap: mem::MaybeUninit<BITMAP> = mem::MaybeUninit::uninit();
      if GetObjectW(info.hbmColor, bitmap_size, Some(bitmap.as_mut_ptr().cast())) != bitmap_size {
        panic!("Cannot object size is not equal to bitmap size");
      }

      let bitmap = bitmap.assume_init_ref();

      let width = bitmap.bmWidth as usize;
      let height = bitmap.bmHeight as usize;

      let buf_size = width
        .checked_mul(height)
        .and_then(|size| size.checked_mul(4))
        .expect("Cannot get buffer size");
      let mut buf: Vec<u8> = Vec::with_capacity(buf_size);

      let dc = GetDC(HWND(ptr::null_mut()));
      let mut bitmap_info = BITMAPINFOHEADER {
        biSize: biheader_size,
        biWidth: bitmap.bmWidth,
        biHeight: -bitmap.bmHeight,
        biPlanes: 1,
        biBitCount: 32,
        ..BITMAPINFOHEADER::default()
      };
      GetDIBits(
        dc,
        info.hbmColor,
        0,
        bitmap.bmHeight as u32,
        Some(buf.as_mut_ptr().cast()),
        ptr::addr_of_mut!(bitmap_info).cast(),
        DIB_RGB_COLORS,
      );
      buf.set_len(buf.capacity());

      if ReleaseDC(HWND(std::ptr::null_mut()), dc) != 1 {
        panic!("Cannot release Device Context")
      }

      DeleteObject(info.hbmColor).expect("Cannot delete object");

      let mut rgba_data = vec![];
      for chunk in buf.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        mem::swap(b, r);

        rgba_data.push(chunk.to_vec());
      }

      rgba_data
    }
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe { CoUninitialize() };
  }
}
