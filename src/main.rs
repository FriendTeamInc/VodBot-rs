// VodBot (c) 2020-23 Logan "NotQuiteApex" Hickok-Dickson

extern crate clap;
extern crate dirs;

mod cli;
mod config;
mod gql;
mod itd;
mod twitch;
mod twitch_api;
mod util;
mod vodbot_api;
mod commands {
    pub mod info;
    pub mod init;
    pub mod pull;
}

use crate::cli::{Cli, Commands};

use clap::Parser;

fn deffered_main() -> Result<(), util::ExitMsg> {
    // Setup the SIGINT handler
    ctrlc::set_handler(move || {
        eprintln!("Interrupted!");
        eprintln!(
            "Exit code: {:?} ({})",
            util::ExitCode::Interrupted,
            util::ExitCode::Interrupted as i32
        );
        std::process::exit(util::ExitCode::Interrupted as i32);
    })
    .map_err(|why| util::ExitMsg {
        code: util::ExitCode::CannotRegisterSignalHandler,
        msg: format!(
            "Cannot register signal interrupt handler, reason: \"{}\".",
            why
        ),
    })?;

    // Parse command line arguments
    let args = Cli::parse();

    // Figure out what config path to use
    let config_path = args
        .config_path
        .unwrap_or(config::default_config_location());

    // Run various commands
    match args.command {
        Commands::Init { overwrite_confirm } => commands::init::run(overwrite_confirm)?,
        Commands::Info { json, strings } => commands::info::run(config_path, json, strings)?,
        Commands::Pull { mode } => commands::pull::run(config_path, mode)?,
        Commands::Stage { command } => {
            println!("stage! {:?}", command);
        }
        Commands::Export { stage_id, path } => {
            println!("export! {} {}", stage_id, path.to_str().unwrap());
        }
        Commands::Upload { stage_id } => {
            println!("upload! {}", stage_id);
        }
    }

    // We're done here!
    Ok(())
}

fn main() {
    std::process::exit(deffered_main().map_or_else(
        |err| {
            let code: i32 = err.code.clone() as i32;
            eprintln!(
                "{}\nExit code: {:?} ({})",
                err.msg.as_str(),
                &err.code,
                code
            );
            code
        },
        |_| 0,
    ))
}
