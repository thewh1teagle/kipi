use crate::args::Args;
use eyre::Result;
use humantime::Duration;

/*
**MacOS**

View scheduled wake-ups with:
    sudo pmset -g sched

Set a scheduled wake-up (one-time event) with:
    sudo pmset schedule wakeorpoweron "MM/DD/YY HH:MM:SS"

Cancel a scheduled event with:
    sudo pmset schedule cancel <event_id>

Note: The event will be automatically deleted after it is triggered.
*/
#[cfg(target_os = "macos")]
pub fn scheduled_wakeup(schedule: Duration) -> Result<()> {
  use chrono::Local;
  use eyre::bail;
  use std::process::Command;
  let now = Local::now();
  let wakeup_time = now + std::time::Duration::from_secs(schedule.as_secs());
  let formatted_time = wakeup_time.format("%m/%d/%y %H:%M:%S").to_string();

  let args = ["schedule", "wakeorpoweron", &formatted_time];
  log::debug!("args: {:?}", args);
  let output = Command::new("pmset").args(args).output()?;
  if !output.status.success() {
    bail!(
      "pmset failed with status: {}",
      output.status.code().unwrap_or(-1)
    );
  }
  Ok(())
}

/*
**Windows**

References:
- Detailed guide: https://www.codeproject.com/Tips/628562/How-to-wake-up-a-PC-using-waitable-timer
- Verify active wake timers using: `powercfg /waketimers`

Notes:
- This functionality requires a motherboard that supports `Wake-up on RTC`.
- A potential delay of up to 15 seconds may occur due to BIOS scheduling.
*/
#[cfg(windows)]
pub fn scheduled_wakeup(schedule: Duration) -> Result<()> {
  use windows::Win32::Foundation::HANDLE;
  use windows::Win32::System::Power::{SetThreadExecutionState, ES_DISPLAY_REQUIRED};
  use windows::Win32::System::Threading::{
    CreateWaitableTimerW, SetWaitableTimer, WaitForSingleObject, INFINITE,
  };

  // Due time in 100-nanosecond intervals (negative for relative time)
  let due_time = -(schedule.as_secs() as i64 * 10_000_000);
  unsafe {
    let timer: HANDLE = CreateWaitableTimerW(None, true, None)?;
    SetWaitableTimer(
      timer, &due_time, 0, // One-time timer
      None, None, true, // Resume from sleep
    )?;
    WaitForSingleObject(timer, INFINITE);
    // Turn on the monitor
    SetThreadExecutionState(ES_DISPLAY_REQUIRED);
  };
  Ok(())
}

pub fn run(args: &Args) -> Result<()> {
  let schedule: Duration = args.schedule.unwrap();

  #[cfg(not(any(windows, target_os = "macos")))]
  panic!("Schedule wakeup is supported only in Windows");

  #[cfg(any(windows, target_os = "macos"))]
  scheduled_wakeup(schedule)
}
