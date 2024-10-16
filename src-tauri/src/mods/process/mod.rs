#![allow(dead_code)]

use anyhow::Result;
use windows::Win32::{
  Foundation::{CloseHandle, MAX_PATH},
  System::{
    ProcessStatus::{EnumProcesses, GetModuleFileNameExW},
    Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
  },
};

pub fn get_processes_exec_name() -> Result<Vec<String>> {
  Ok(
    get_processes()?
      .iter()
      .map(|&e| get_process_executable_name(e))
      .filter(|p_name| !p_name.is_empty())
      .collect(),
  )
}

fn get_processes() -> Result<Vec<u32>> {
  let mut pids = [0; 2048];
  let mut size = 0;

  unsafe { EnumProcesses(pids.as_mut_ptr(), 2048, &mut size)? };

  Ok(pids[0..(size / 4) as usize].to_vec())
}

pub fn get_process_executable_path(pid: u32) -> String {
  unsafe {
    let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
    if handle.is_ok() {
      let handle = handle.ok().unwrap();
      let mut lpbasename = [0u16; MAX_PATH as usize];
      GetModuleFileNameExW(handle, None, &mut lpbasename);

      let _ = CloseHandle(handle);

      return String::from_utf16_lossy(&lpbasename).replace('\0', "").to_string();
    }
  }

  String::new()
}

pub fn get_process_executable_name_from_path(path: &str) -> String {
  let last_backslash = path.rfind('\\');
  if let Some(index) = last_backslash {
    return path[index + 1..].to_string();
  }

  String::new()
}

pub fn get_process_executable_name(pid: u32) -> String {
  let path = get_process_executable_path(pid);
  get_process_executable_name_from_path(&path)
}

pub fn get_processes_by_name(name: &str) -> Result<Vec<String>> {
  let pids = get_processes()?;

  Ok(
    pids
      .iter()
      .map(|&e| get_process_executable_name(e))
      .filter(|p_name| p_name == &name.to_lowercase())
      .filter(|p_name| !p_name.is_empty())
      .collect(),
  )
}
