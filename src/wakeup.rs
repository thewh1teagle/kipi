use crate::args::Args;
use eyre::Result;
use humantime::Duration;

/*
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

  #[cfg(not(windows))]
  panic!("Schedule wakeup is supported only in Windows");

  #[cfg(windows)]
  scheduled_wakeup(schedule)
}
