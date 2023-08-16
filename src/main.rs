// VodBot (c) 2020-23 Logan "NotQuiteApex" Hickok-Dickson

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
        let e = util::ExitMsg::new(util::ExitCode::Interrupted, "Interrupted!".to_owned());
        println!(
            " Interrupted!\nExit code: {:?} ({})",
            e.code.clone(),
            e.code.clone() as i32
        );
        std::process::exit(e.code as i32);
    })
    .map_err(|why| {
        util::ExitMsg::new(
            util::ExitCode::CannotRegisterSignalHandler,
            format!(
                "Cannot register signal interrupt handler, reason: \"{}\".",
                why
            ),
        )
    })?;

    // Parse command line arguments
    let args = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .timestamp(stderrlog::Timestamp::Millisecond)
        .verbosity(args.verbose as usize)
        .init()
        .map_err(|e| {
            util::ExitMsg::new(
                util::ExitCode::StderrLoggerError,
                format!(
                    "Failed to initialize stderr logger, reason: \"{}\".",
                    e.to_string()
                ),
            )
        })?;

    // Figure out what config path to use
    let config_path = args
        .config_path
        .unwrap_or(config::default_config_location());

    // Run various commands
    log::trace!("args.command: {:?}", args.command);
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
            println!("{}", err);
            err.code as i32
        },
        |_| 0,
    ))
}
