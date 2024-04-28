use args::{parse_args, Args};
use std::{process, thread, time::Duration};
mod args;
use eyre::Result;

fn setup(args: &Args) -> Result<()> {
  // Graceful exit
  ctrlc::set_handler(move || {
    // Disable keepawake
    log::debug!("Detect Ctrl+C");
    keepawake::Builder::default()
      .display(false)
      .idle(false)
      .sleep(false)
      .create()
      .expect("Error setting keepawake");

    process::exit(0);
  })?;

  // Keep OS awake based on args
  log::debug!("Setup keepawake with {:?}", args);
  keepawake::Builder::default()
    .display(args.display)
    .idle(args.os)
    .sleep(args.os)
    .create()?;
  Ok(())
}

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
  setup(&args)?;
  keep_run(&args);
  Ok(())
}
