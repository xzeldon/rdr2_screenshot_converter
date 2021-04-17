mod buffertrim;
mod console;

use rdr2_screenshot_converter::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = Config::new().unwrap();

    if args.len() >= 2 {
        config.set_custom_export_path(&args).unwrap();
    }

    let _ansi_support = console::enable_ansi_support();

    if let Err(e) = rdr2_screenshot_converter::run(config) {
        eprintln!("{}", e);
    }

    let _ctrl_c_handler = console::set_ctrl_c_handler();
}