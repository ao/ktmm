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
# Optionally: Add this official tap (only needed once)
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

## Development

### Version Management

This project includes a script to easily update the version across all files:

```bash
# Update to version 0.5.0
./scripts/update_version.sh 0.5.0
```

This script will:
1. Update the version in `Cargo.toml`
2. Update the version in `Formula/ktmm.rb`
3. Update the version in `Formula/ktmm_cask_style.rb`

After running the script, follow the displayed instructions to commit, tag, and push the changes.

### Automatic Formula Updates

This project is configured to automatically update the Homebrew formulas whenever a new release is created. The GitHub Actions workflow:

1. Builds binaries for all supported platforms
2. Creates a GitHub release with the binaries
3. Calculates SHA256 checksums for all binaries and the source tarball
4. Updates both formula files with the new version and checksums:
   - `Formula/ktmm.rb` (binary formula for personal tap)
   - `Formula/ktmm_cask_style.rb` (source-based formula for potential Homebrew core submission)
5. Commits and pushes the changes back to the main branch

This eliminates the need for manual updates to the formulas when releasing new versions. The complete release process is:

```bash
# 1. Update the version
./scripts/update_version.sh 0.5.0

# 2. Commit the changes
git commit -am "Bump version to 0.5.0"

# 3. Create and push a tag
git tag -a v0.5.0 -m "Release v0.5.0"
git push origin main && git push origin v0.5.0
```

The GitHub Actions workflow will handle the rest automatically.