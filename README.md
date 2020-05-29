# cicero-tui

A Unicode Tool with terminal terminal user interface.

[![](https://github.com/eyeplum/cicero-tui/workflows/CI/badge.svg)](https://github.com/eyeplum/cicero-tui/actions)

## Build Requirements

### Rust

Make sure a Rust toolchain is installed. See [rustup](https://rustup.rs/).

### fontconfig and freetype

This tool requires `fontconfig` and `freetype` libraries to build.

- On GNU/Linux (Tested on Ubuntu 18.04 LTS)

    ```sh
    # Assuming a C/C++ compiler and CMake are installed
    # This installs shared libraries for both fontconfig and freetype
    $ sudo apt install libfontconfig1-dev
    ```

- On macOS

    ```sh
    # Assuming homebrew is installed
    $ brew install pkg-config fontconfig
    ```

## License
[![](images/gplv3.png)](https://www.gnu.org/licenses/gpl-3.0.html)