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