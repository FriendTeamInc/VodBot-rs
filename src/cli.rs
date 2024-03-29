// CLI setup

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "VodBot", author, version)]
#[command(about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
    #[arg(short, long)]
    pub no_color: bool,
    #[arg(short, long)]
    pub update_cache: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Initialize directories and files for VodBot")]
    Init {
        #[arg(short = 'y', help = "Confirm overwriting an existing config")]
        overwrite_confirm: bool,
    },
    #[command(about = "Get info about videos, clips, or channels")]
    Info {
        // JSON output
        #[arg(short, long)]
        json: bool,

        strings: Vec<String>,
    },
    #[command(about = "Pull videos, clips, chat logs, and more")]
    Pull {
        #[arg(value_enum, default_value_t=PullMode::All)]
        mode: PullMode,
    },
    #[command(about = "Stage video data for export or upload")]
    Stage {
        // command: StageCommands,
        #[command(subcommand)]
        command: StageMode,
    },
    #[command(about = "Export staged data to local storage")]
    Export { stage_id: String, path: PathBuf },
    #[command(about = "Upload staged data to YouTube")]
    Upload { stage_id: String },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum PullMode {
    Vods,
    Clips,
    All,
}

#[derive(Debug, Subcommand)]
pub enum StageMode {
    #[command(about = "Create a new stage of video data")]
    New {
        #[arg(help = "ID(s) of videos (VODs, Clips, etc)")]
        ids: Vec<String>,

        #[arg(long, help = "Names of the channels involved in the video")]
        streamers: Option<Vec<String>>,
        #[arg(long, help = "Title of the final video")]
        title: Option<String>,
        #[arg(long, help = "Description of the final video")]
        description: Option<String>,
        #[arg(long, help = "Starting time of video slice")]
        ss: Option<Vec<String>>,
        #[arg(long, help = "Ending time of video slice")]
        to: Option<Vec<String>>,
        // #[arg(long)]
        // tn_heads: Option<Vec<String>>,
        // #[arg(long)]
        // tn_game: Option<String>,
        // #[arg(long)]
        // tn_text: Option<String>,
        // #[arg(long)]
        // tn_video_idx: Option<usize>,
        // #[arg(long)]
        // tn_timestamp: Option<String>,
    },
    #[command(about = "Remove staged data")]
    Remove {
        #[arg(help = "ID(s) of staged data")]
        ids: Vec<String>,

        #[arg(short = 'y', help = "Confirm removal")]
        confirm: bool,
    },
    #[command(about = "List current staged data")]
    List {
        #[arg(help = "ID(s) of staged data")]
        ids: Option<Vec<String>>,
    },
}
