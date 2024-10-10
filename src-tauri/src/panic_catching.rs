use std::{backtrace::Backtrace, cell::Cell, panic};

use windows::{
  core::{w, BSTR},
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL},
  },
};

thread_local! {
  pub static BACKTRACE: Cell<Option<Backtrace>> = const {Cell::new(None)};
}

pub fn init() {
  panic::set_hook(Box::new(|_| {
    let trace = Backtrace::force_capture();
    BACKTRACE.with(move |b| b.set(Some(trace)));
  }));
}

pub fn trace<F, T>(f: F)
where
  F: FnOnce() -> T + Send + 'static + panic::UnwindSafe,
  T: Send + 'static,
{
  if let Err(error) = panic::catch_unwind(f) {
    let trace = BACKTRACE.with(|b| b.take()).unwrap().to_string();

    let mut trimmed_trace = trace
      .split("\n")
      .filter(|e| e.contains("at ") && !e.contains("/rustc/") && e.contains(".rs"))
      .map(|e| format!("{:>4}", "") + e.trim())
      .collect::<Vec<_>>()
      .join("\n");

    if trimmed_trace.is_empty() {
      trimmed_trace = "- Trace:\n".to_string() + "No backtrace available";
    } else {
      trimmed_trace = "- Trace:\n".to_string() + &trimmed_trace;
    }

    let error = if let Some(error) = error.downcast_ref::<String>() {
      format!("- Error: {}", error)
    } else if let Some(error) = error.downcast_ref::<&str>() {
      format!("- Error: {}", error)
    } else {
      "- Error: Unknown".to_string()
    };

    unsafe {
      MessageBoxW(
        HWND::default(),
        &BSTR::from(&format!("{error}\n{trimmed_trace}\n\nThe program will now exit")),
        w!("PwccaAutoGUI"),
        MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
      )
    };

    std::process::exit(1);
  }
}
