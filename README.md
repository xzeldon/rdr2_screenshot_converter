# RDR2 Screenshot converter [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

Convert and save photomode screenshots from Red Dead Redemption 2 to JPEG format.

![Imgur](https://i.imgur.com/ZGbmHYd.png)

Mirror on my [<img src="https://git.zeldon.ru/assets/img/logo.svg" align="center" width="20" height="20"/> Git](https://git.zeldon.ru/zeldon/rdr2_screenshot_converter)

## QuickStart

Just [download](https://github.com/xzeldon/rdr2_screenshot_converter/releases) the executable file from releases and run it. It will automatically find your screenshots and save them.

## Arguments

You can define a path for saving screenshots. To do this, open a command prompt, specify the path to the executable file and the path to save the screenshots.

```
cli.exe C:\screenshots
```

This command will save screenshots to ```C:\screenshots```

## Building

All instructions tested on Windows 10 Pro for workstations 19042.928.

You need:

* [Rust](https://www.rust-lang.org)

```
cargo build --release --bin cli
```
