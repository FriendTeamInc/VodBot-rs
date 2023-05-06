// VodBot (c) 2020-23 Logan "NotQuiteApex" Hickok-Dickson

use std::path::Path;

extern crate clap;
extern crate dirs;

mod util;

fn deffered_main() -> Result<(), util::ExitMsg> {
	// Load the environment variables from cargo for this info.
	const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
	const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

    println!("{} {}", AUTHORS.unwrap_or("UNKNOWN"), VERSION.unwrap_or("UNKNOWN"));

    Ok(())
}

fn main() {
    println!("Hello, world!");

    std::process::exit(
        deffered_main()
        .map_or_else(|err| {
            print!("{}", err.msg.as_str());
            err.code as i32
        }, |_| 0)
    )
}
