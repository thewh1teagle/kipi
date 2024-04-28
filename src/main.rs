use args::{parse_args, Args};
use std::{process, thread, time::Duration};
mod args;

fn setup(args: &Args) {
  // Graceful exit
  ctrlc::set_handler(move || {
    // Disable keepawake
    log::debug!("Detect Ctrl+C");
    keepawake::Builder::default()
      .display(false)
      .idle(false)
      .sleep(false);
    process::exit(0);
  })
  .expect("Error setting Ctrl+C handler");

  // Keep OS awake based on args
  log::debug!("Setup keepawake with {:?}", args);
  keepawake::Builder::default()
    .display(args.display)
    .idle(args.os)
    .create()
    .expect("Error setting keepawake");
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

fn main() {
  env_logger::init();
  let args = parse_args();
  setup(&args);
  keep_run(&args);
}
