# KTMM (Keep That Mouse Moving)

[![Release](https://github.com/ao/ktmm/actions/workflows/release.yml/badge.svg)](https://github.com/ao/ktmm/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust cross-platform implementation of the "Keep That Mouse Moving" utility that prevents system sleep/idle mode by making periodic, subtle mouse movements.

## Purpose

KTMM prevents your system from going to sleep or activating the screensaver by simulating minimal mouse activity. It's useful for:

- Preventing screen locks during presentations
- Keeping remote desktop connections alive
- Avoiding automatic status changes in messaging applications
- Bypassing idle timeouts in various applications

## How It Works

The application runs in an infinite loop that:

1. Sleeps for 60 seconds
2. Gets the current mouse position (x, y)
3. Moves the mouse 1 pixel right and 1 pixel down (x+1, y+1)
4. Sleeps for 6 milliseconds
5. Moves the mouse back to the original position (x, y)
6. Repeats indefinitely

The mouse movements are minimal and designed to be barely noticeable during normal computer use.

## Features

- **Cross-platform compatibility**: Works on Windows, macOS, and Linux
- **No user input required**: Runs silently in the background
- **Minimal resource usage**: Low CPU and memory footprint
- **No configuration needed**: Works out of the box
- **Non-intrusive**: Mouse movements are barely noticeable

## Installation

### Homebrew (macOS)

```bash
# Add the tap (only needed once)
brew tap ao/ktmm https://github.com/ao/ktmm

# Install KTMM
brew install ktmm
```

### Manual Installation

#### Pre-built Binaries

Download the latest release for your platform from the [GitHub Releases page](https://github.com/ao/ktmm/releases).

Available binaries:
- macOS (Intel): `ktmm-macos-x86_64`
- macOS (Apple Silicon): `ktmm-macos-arm64`
- Linux (x86_64): `ktmm-linux-amd64`
- Windows (x86_64): `ktmm-windows-amd64.exe`

Make the binary executable (macOS/Linux):
```bash
chmod +x ktmm-*
```

#### Building from Source

##### Prerequisites

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- **Linux only**: X11 development libraries

##### Linux System Dependencies

KTMM requires X11 development libraries on Linux systems. Install them using your distribution's package manager:

**Ubuntu/Debian**:
```bash
sudo apt-get update
sudo apt-get install -y libx11-dev libxtst-dev libxinerama-dev libxrandr-dev libxss-dev libxdo-dev
```

**Fedora/RHEL/CentOS**:
```bash
sudo dnf install -y libX11-devel libXtst-devel libXinerama-devel libXrandr-devel libXScrnSaver-devel libxdo-devel
```

**Arch Linux**:
```bash
sudo pacman -S libx11 libxtst libxinerama libxrandr libxss xdotool
```

**Note**: KTMM is designed for X11 and may have limited functionality under Wayland.

##### Build

```bash
git clone https://github.com/ao/ktmm.git
cd ktmm
cargo build --release
```

### Running KTMM

```bash
# If installed via Homebrew
ktmm

# If using pre-built binary
./ktmm-macos-x86_64  # or appropriate binary name for your platform

# If built from source
cargo run --release
# Or directly run the compiled binary
./target/release/ktmm  # On macOS/Linux
.\target\release\ktmm.exe  # On Windows
```

## System Requirements

- Any operating system supported by Rust (Windows, macOS, Linux)
- Minimal CPU and memory resources

## License

MIT