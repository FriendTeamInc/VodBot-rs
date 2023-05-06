// VodBot (c) 2020-23 Logan "NotQuiteApex" Hickok-Dickson

extern crate clap;
extern crate dirs;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

mod config;
mod util;

#[derive(Debug, Parser)]
#[command(name = "VodBot", author, version)]
#[command(about = "A video and chat manager for Twitch.tv", long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about="Initialize directories and files for VodBot")]
    Init {

    },
    #[command(about="Get info about videos, clips, or channels")]
    Info {

    },
    #[command(about="Pull videos, clips, chat logs, and more")]
    Pull {
        #[arg(value_enum, default_value_t=PullMode::All)]
        mode: PullMode,
    },
    #[command(about="Stage video data for export or upload")]
    Stage {
        // command: StageCommands,
    },
    #[command(about="Export staged data to local storage")]
    Export {
        
    },
    #[command(about="Upload staged data to YouTube")]
    Upload {
        
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum PullMode {
    Vods,
    Clips,
    All,
}


fn deffered_main() -> Result<(), util::ExitMsg> {
    // Testing the CLI args
    let args = Cli::parse();
    println!("{:?}", args);

    Ok(())
}

fn main() {
    std::process::exit(deffered_main().map_or_else(
        |err| {
            print!("{}", err.msg.as_str());
            err.code as i32
        },
        |_| 0,
    ))
}
