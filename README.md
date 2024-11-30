# kipi

Keep your operating system or display awake

Supports: `Windows` / `Linux` / `macOS`

# Download

You can download `kipi` from this [website](https://thewh1teagle.github.io/kipi/)

# Usage

```console
Keep your OS awake

Usage: kipi.exe [OPTIONS]

Options:
  -o, --os                   Keep the operating system awake. Defaults to true
  -d, --display              Keep the display awake. Defaults to false
      --duration <DURATION>  Duration for which to keep the system awake. Optional. Example: --duration 1h30m (keeps awake for 1 hour and 30 minutes)
      --schedule <SCHEDULE>  Schedule when to wakeup the PC from sleep. Optional. Example: --schedule 1m (schedule wake up in one minute)
  -v, --version              Get version of the program
      --report-issue         Report an issue in Github
      --update               Update kipi and exit
  -h, --help                 Print help
```

# Update

You can update `kipi` by execute

```console
kipi-update
```

## Gotchas

On `Windows` when using `--schedule` for wakeup schedule, you may need to enable wake up timers by opening `run.exe` -> `control powercfg.cpl,,3` -> `Sleep` -> `Allow wake timers` -> `Enable` instead of `Important only`. To verify it's working run `powercfg /waketimers`