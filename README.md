# cicero [![Build and Test](https://github.com/eyeplum/cicero-tui/workflows/Build%20and%20Test/badge.svg)](https://github.com/eyeplum/cicero-tui/actions?query=workflow%3A%22Build+and+Test%22)

Unicode tool with a terminal user interface.

![Screenshot](images/screenshot.png)

## Usage

```
$ cicero -h
Cicero: A Unicode Tool 0.2.1 (Unicode Version 13.0.0)

USAGE:
    cicero.exe [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -u                           Parses INPUT as comma separated code points,
                                 same as '--input-type=code-points',
                                 ignored if '--input-type' is specified
    -g, --generate-flamegraph    Generate Flamegraph for all Unicode Planes,
                                 which can be loaded by Chrome's tracer UI (about:tracing)
    -h, --help                   Prints help information
    -t, --tui                    Shows Terminal UI
    -V, --version                Prints version information

OPTIONS:
    -i, --input-type <TYPE>         Specifies input type, 'string' by default,
                                    valid values: string, code-points
    -o, --output-format <FORMAT>    Specifies output format, 'text' by default,
                                    valid values: text, json

ARGS:
    <INPUT>    a string or comma separated code points
```

## Supported Platforms

Cicero is tested on GNU/Linux, macOS, and Windows.

## Installation

### Homebrew Tap

The easiest way to install Cicero is via homebrew tap.

```sh
$ brew install eyeplum/tap/cicero-tui
```

Or alternatively:

```sh
$ brew tap eyeplum/tap
$ brew install cicero-tui
```

### AUR

`cicero` can be installed from available [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=cicero&outdated=&SB=n&SO=a&PP=50&do_Search=Go) using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example,

```sh
$ yay -S cicero
```

If you prefer, you can clone the [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=cicero&outdated=&SB=n&SO=a&PP=50&do_Search=Go) and then compile them with [makepkg](https://wiki.archlinux.org/index.php/Makepkg). For example,

```sh
$ git clone https://aur.archlinux.org/cicero.git
$ cd cicero
$ makepkg -si
```

### Building From Source

You can also build Cicero from source.

#### Rust

Make sure the latest stable Rust toolchain is installed. See [rustup](https://rustup.rs/).

#### fontconfig and freetype (macOS and Linux only)

Cicero requires `fontconfig` and `freetype` libraries to build on Unix platforms.

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

#### Building

Building cicero should be as easy as:

```sh
$ git clone https://github.com/eyeplum/cicero-tui
$ cd cicero-tui
$ cargo build --release
$ ./target/release/cicero --version
Cicero: A Unicode Tool 0.1.0 (Unicode Version 13.0.0)
```

You could also copy the final product to somewhere in your `PATH`, for example:

```sh
$ cp target/release/cicero $HOME/.local/bin/
```

## Configuration

A `settings.toml` file can be created to configure character previews of Cicero.

**Note: The configuration file is required on Windows in order to preview characters.**

The file is read from the following locations:
- On Unix platforms, the file is read from `$HOME/.config/cicero/settings.toml`
- On Windows, the file is read from `C:\Users\<username>\.config\cicero\settings.toml`

The configuration file has the following format:

```toml
# Whether to use fontconfig for font discovery.
# True by default.
# Ignored on Windows (as fontconfig is only available on Unix platforms).
use_fontconfig = false 

# Paths for recursively searching for fonts.
# Required on Windows in order to preview characters.
# Must be absolute paths.
# Ignored if fontconfig is set to true.
font_search_paths = ["<path>"]

# Preview fonts configuration.
# Optional. If omitted all discovered fonts are used in character preview.
# Multiple entries can be defined, the final fonts used in character preview
# are a union of all fonts matched by all entries.
[[preview_fonts]]
code_point_range = "<range>" # Code point range to apply this entry, supported formats:
                             # - An inclusive range of Unicode Code Points, e.g. "U+0020..U+00FF"
                             # - Unicode Block name, e.g. "Basic Latin"
                             # - Unicode Plane name, e.g. "Basic Multilingual Plane"
                             # This filed is optional, if omitted this entry will be applied
                             # to all Unicode characters.
font_name = "<font>" # Name of the font to be added to the preview list.
                     # The name is partially matched to the font's family name and full name.
```

### `settings.toml` Examples

#### Unix

##### Use `fontconfig`

This config file has the following behaviors:
- Use `fontconfig` for font discovery
- Use `Noto Sans` for displayable ASCII characters
- Use all fonts with `CJK` in their name for characters in the `CJK Unified Ideographs` block
- Use all discovered fonts for other characters

```toml
[[preview_fonts]]
code_point_range = "U+0020..U+007E"
font_name = "Noto Sans"

[[preview_fonts]]
code_point_range = "CJK Unified Ideographs"
font_name = "CJK"
```

##### Don't use `fontconfig`

This config file has the following behaviours:
- Don't use `fontconfig` for font discovery 
- Search fonts recursively in `~/Fonts/` and `~/Documents/Fonts/` directory
- Use `Noto Sans` for displayable ASCII characters
- Use all fonts with `CJK` in their name for characters in the `CJK Unified Ideographs` block
- Use all discovered fonts for other characters

```toml
use_fontconfig = false

font_search_paths = [
  "/home/<username>/Fonts", 
  "/home/<username>/Documents/Fonts"
]

[[preview_fonts]]
code_point_range = "U+0020..U+007E"
font_name = "Noto Sans"

[[preview_fonts]]
code_point_range = "CJK Unified Ideographs"
font_name = "CJK"
```


#### Windows

- Search fonts recursively in `C:\Windows\Fonts` and `C:\Users\<username>\Documents\Fonts` directory
- Use `Noto Sans` for displayable ASCII characters
- Use all fonts with `CJK` in their name for characters in the `CJK Unified Ideographs` block
- Use all discovered fonts for other characters

```toml
font_search_paths = [
  "C:\\Windows\\Fonts",
  "C:\\Users\\<username>\\Documents\\Fonts",
]

[[preview_fonts]]
code_point_range = "U+0020..U+007E"
font_name = "Noto Sans"

[[preview_fonts]]
code_point_range = "CJK Unified Ideographs"
font_name = "CJK"
```

## License

[![](images/gplv3.png)](https://www.gnu.org/licenses/gpl-3.0.html)
