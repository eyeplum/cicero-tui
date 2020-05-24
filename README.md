# cicero-tui

A Unicode Tool with Terminal UI

[![](https://github.com/eyeplum/cicero-tui/workflows/CI/badge.svg)](https://github.com/eyeplum/cicero-tui/actions)

## Build

### Requirements

#### Rust

Make sure a Rust toolchain is installed.

#### fontconfig and freetype

This tool requires `fontconfig` and `freetype` libraries to build.

##### GNU/Linux (Tested on Ubuntu 18.04 LTS)

```sh
# (Optional) You may need to install a C/C++ compiler and CMake if they are not installed already
$ sudo apt install build-essential cmake

# This installs shared libraries for both fontconfig and freetype
$ sudo apt install libfontconfig1-dev
```

##### macOS

```sh
$ brew install fontconfig
```
