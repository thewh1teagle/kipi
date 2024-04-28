use args::{parse_args, Args};
use std::{process, thread, time::Duration};
mod args;
use eyre::Result;

/// Keep thread alive based on args
fn keep_run(args: &Args) {
  if let Some(duration) = args.duration {
    log::debug!("Keep thread alive for {}", duration);
    thread::sleep(*duration);
  } else {
    log::debug!("Keep thread alive infinite");
    loop {
      thread::sleep(Duration::from_secs(1));
    }
  }
}

fn main() -> Result<()> {
  env_logger::init();
  let args = parse_args()?;
  // Graceful exit
  ctrlc::set_handler(move || {
    // Disable keepawake
    log::debug!("Detect Ctrl+C");
    let _keep = keepawake::Builder::default()
      .display(false)
      .idle(false)
      .sleep(false)
      .create()
      .expect("Error setting keepawake");
    process::exit(0);
  })?;

  // Keep OS awake based on args
  log::debug!("Setup keepawake with {:?}", args);
  let _keep = keepawake::Builder::default()
    .reason(if args.display {
      "Keep display on"
    } else {
      "Keep OS awake"
    })
    .app_name(env!("CARGO_PKG_NAME"))
    .display(args.display)
    .idle(args.os)
    .sleep(args.os)
    .create()?;
  keep_run(&args);
  Ok(())
}
