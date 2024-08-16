#[derive(Debug)]
pub enum Events {
  Startup,

  Discord,
  Ethernet,
  Taskbar,

  TurnOffMonitor,
  RefreshRate,

  Exit,
}

impl Into<String> for Events {
  fn into(self) -> String {
    format!("{:?}", self)
  }
}
