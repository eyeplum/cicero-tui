# cicero

A Unicode Tool with terminal user interface.

## Usage

```
$ cicero -h
Cicero: A Unicode Tool 0.1.0 (Unicode Version 13.0.0)

USAGE:
    cicero [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -u               Parse INPUT as comma seperated code points,
                     same as '--input-type=code-points',
                     ignored if '--input-type' is specified
    -h, --help       Prints help information
    -t, --tui        Show Terminal UI
    -V, --version    Prints version information

OPTIONS:
    -i, --input-type <TYPE>         Specify input type, 'string' by default,
                                    valid values: string, code-points
    -o, --output-format <FORMAT>    Specify output format, 'text' by default,
                                    valid values: text, json

ARGS:
    <INPUT>    a string or comma separated code points
```

## Install (build from source)

Currently the only way to install cicero is by building from source.

### Supported Platforms

Cicero is tested on GNU/Linux and macOS.

Building on Windows is not possible at the moment, but Windows Subsystem for Linux should work.

### Rust

Make sure the latest stable Rust toolchain is installed. See [rustup](https://rustup.rs/).

### fontconfig and freetype

Cicero requires `fontconfig` and `freetype` libraries to build. In order to install them:

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

### Build

Building cicero should be as easy as:

```
$ cargo build --release
```

You could also copy the final product to somewhere in your `PATH`, for example:

```
$ cp target/release/cicero $HOME/.local/bin/
```

## License
[![](images/gplv3.png)](https://www.gnu.org/licenses/gpl-3.0.html)