# kipi

Keep your operating system or display awake

Supports: `Windows` / `Linux` / `macOS`

# Download

You can download `kipi` from this [website](https://thewh1teagle.github.io/kipi/)

Alternatively, by execute:

## Windows

```console
powershell -c "irm https://github.com/thewh1teagle/kipi/releases/latest/download/kipi-installer.ps1 | iex"
```

## Linux / macOS

```console
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/thewh1teagle/kipi/releases/latest/download/kipi-installer.sh | sh
```

# Update

You can update `kipi` by execute

```console
kipi-update
```

# Usage

```console
Keep your OS awake

Usage: kipi [OPTIONS]

Options:
  -o, --os                   Keep the operating system awake. Defaults to true
  -d, --display              Keep the display awake. Defaults to false
      --duration <DURATION>  Duration for which to keep the system awake. Optional. Example: --duration 1h30m (keeps awake for 1 hour and 30 minutes)
  -v, --version              Get version of the program
      --report-issue         Report an issue in Github
  -h, --help                 Print help
```
