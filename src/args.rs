use clap::Parser;
use std::process;

/// Keep your OS awake
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
  /// Keep the operating system awake. Defaults to true.
  #[arg(short, long, default_value_t = true)]
  pub os: bool,

  /// Keep the display awake. Defaults to false.
  #[arg(short, long, default_value_t = false)]
  pub display: bool,

  /// Duration for which to keep the system awake. Optional.
  #[arg(long)]
  pub duration: Option<humantime::Duration>, // Optional duration

  /// Get version of the program
  #[arg(short, long, default_missing_value = "true")]
  pub version: bool,

  /// Report an issue in Github
  #[arg(long)]
  report_issue: bool,
}

/// Return new issue URL with OS info
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

/// Encode url. same as encodeURIComponent
fn encode_url(url: String) -> String {
  url::form_urlencoded::byte_serialize(url.as_bytes()).collect()
}

/// Open issue URL in Github with passed information and exit
fn report_issue() {
  let url = issue_url();
  log::debug!("open url {}", url);
  open::that(url).expect("Can't open URL in browser");
  process::exit(0);
}

/// Print version of the program and exit
fn print_version() {
  println!(
    "{}: {} ({})",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH")
  );
  process::exit(0);
}

/// Parse args with default handlers for version and such
pub fn parse_args() -> Args {
  let args = Args::parse();
  if args.version {
    print_version();
  }
  if args.report_issue {
    report_issue();
  }
  args
}
