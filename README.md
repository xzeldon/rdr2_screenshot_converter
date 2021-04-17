# RDR2 Screenshot converter [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

Convert and save screenshots from rdr2 photo mode to JPEG format

![Imgur](https://i.imgur.com/ZGbmHYd.png)

## QuickStart
Just [download](https://github.com/xzeldon/rdr2_screenshot_converter/releases/download/1.0.0/rdr2_screenshot_converter.exe) the executable file from releases and run it. It will automatically find your screenshots and save them.

## Arguments
You can define a path for saving screenshots. To do this, open a command prompt, specify the path to the executable file and the path to save the screenshots.
```
rdr2_screenshot_converter.exe C:\screenshots
```
This command will save screenshots to ```C:\screenshots```

## Building
All instructions tested on Windows 10 Pro for workstations 19042.928.

You need:
* [Rust](https://www.rust-lang.org)
```
rustup override set nightly
cargo build --release
```
