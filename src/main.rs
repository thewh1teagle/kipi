mod args;
use eyre::Result;
mod keepawake;
mod wakeup;

fn main() -> Result<()> {
  env_logger::init();
  let args = args::parse_args()?;

  if args.schedule.is_some() {
    wakeup::run(&args)?;
  } else {
    crate::keepawake::run(&args)?;
  }

  Ok(())
}
