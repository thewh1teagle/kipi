use clap::Parser;
use std::{env, process, thread, time::Duration};

/// Keep your OS awake
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
  /// Keep the operating system awake. Defaults to true.
  #[arg(short, long, default_value_t = true)]
  os: bool,

  /// Keep the display awake. Defaults to false.
  #[arg(short, long, default_value_t = false)]
  display: bool,

  /// Duration for which to keep the system awake. Optional.
  #[arg(long)]
  duration: Option<humantime::Duration>, // Optional duration

  /// Get version of the program
  #[arg(short, long, default_missing_value = "true")]
  version: bool,

  /// Report an issue in Github
  #[arg(long)]
  report_issue: bool,
}

fn issue_url() -> String {
  let info = os_info::get();
  let body = format!(
    "\n\n\n```console\n{} version {} ({})\nOS: {:?} {:?}\nArch: {:?}\n```",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH"),
    info.os_type(),
    info.version().to_string(),
    info.architecture().unwrap_or("unknown"),
  );
  let title = "[Bug]: ";
  format!(
    "https://github.com/thewh1teagle/kipi/issues/new?title={}&body={}",
    encode_url(title.into()),
    encode_url(body)
  )
}

fn encode_url(url: String) -> String {
  url::form_urlencoded::byte_serialize(url.as_bytes()).collect()
}

fn report_issue() {
  let url = issue_url();
  open::that(url).expect("Can't open URL in browser");
  process::exit(0);
}

fn print_version() {
  println!(
    "{}: {} ({})",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH")
  );
  process::exit(0);
}

fn setup(args: &Args) {
  ctrlc::set_handler(move || {
    // Disable keepawake
    keepawake::Builder::default()
      .display(false)
      .idle(false)
      .sleep(false);
    process::exit(0);
  })
  .expect("Error setting Ctrl+C handler");

  keepawake::Builder::default()
    .display(args.display)
    .idle(args.os)
    .create()
    .expect("Error setting keepawake");
}

fn keep_run(args: &Args) {
  if let Some(duration) = args.duration {
    thread::sleep(*duration);
  } else {
    loop {
      thread::sleep(Duration::from_secs(1));
    }
  }
}

fn main() {
  let args = Args::parse();
  if args.version {
    print_version();
  }
  if args.report_issue {
    report_issue();
  }
  setup(&args);
  keep_run(&args);
}
