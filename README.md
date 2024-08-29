# Mek’ut’erīya

A simple program for handling Ethiopian calendar dates.

## Run it with Nix

```bash
nix run nixpkgs#mekuteriya
````

## Installation

```sh
cargo install --git https://github.com/frectonz/mek-ut-er-ya
```

If you don't have Rust installed, you can use the pre-built binaries from the [releases](https://github.com/frectonz/mek-ut-er-ya/releases) page.

Download the binary for your platform.
- If you are on windows download the file that ends with `x86_64-pc-windows-gnu.zip`
- If you are on linux download one of the files ends with `x86_64-unknown-linux-musl.tar.gz`

If you are on linux, you can use the following command to extract the binary from the archive:

```sh
tar xvf <downloaded_file>
```

## Demo

[![asciicast](https://asciinema.org/a/539058.svg)](https://asciinema.org/a/539058)

## Source

- [Formula for converting Ethiopian date from and to Julian day number](https://www.geez.org/Calendars/)
- [Formula for converting Gregorian date from and to Julian day number](https://quasar.as.utexas.edu/BillInfo/JulianDatesG.html)
- [Test Data](https://www.geez.org/Calendars/EthiopicCalendarTest.java)
