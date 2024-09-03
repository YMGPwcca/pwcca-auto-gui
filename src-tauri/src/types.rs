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

impl From<Events> for String {
  fn from(val: Events) -> Self {
    format!("{:?}", val)
  }
}
