// Init command, initializes all the default folders and files for use

use crate::config;
use crate::util;

use std::fs::File;
use std::io::prelude::*;

pub fn run(overwrite_confirm: bool) -> Result<(), util::ExitMsg> {
    let conf = config::Config {
        ..Default::default()
    };

    println!("Creating default config...");

    let config_path = config::from_vodbot_dir(&["config.json"]);

    if config_path.exists() && !overwrite_confirm {

    }

    util::create_dir(&conf.directories.vods)?;
    util::create_dir(&conf.directories.highlights)?;
    util::create_dir(&conf.directories.uploads)?;
    util::create_dir(&conf.directories.premieres)?;
    util::create_dir(&conf.directories.clips)?;
    util::create_dir(&conf.directories.temp)?;
    util::create_dir(&conf.directories.stage)?;
    util::create_dir(&conf.directories.thumbnail)?;

    let mut config_file = File::create(&config_path).map_err(|why| util::ExitMsg {
        code: util::ExitCode::InitCannotOpenConfig,
        msg: format!(
            "Failed to open file to write config to `{}`, reason: \"{}\".",
            &config_path.display(),
            why
        ),
    })?;

    let json_to_write = serde_json::to_string(&conf).map_err(|why| util::ExitMsg {
        code: util::ExitCode::InitCannotSerializeConfig,
        msg: format!("Failed to serialize config, reason: \"{}\".", why),
    })?;

    config_file
        .write_all(json_to_write.as_bytes())
        .map_err(|why| util::ExitMsg {
            code: util::ExitCode::InitCannotWriteConfig,
            msg: format!(
                "Failed to write config to `{}`, reason: \"{}\".",
                &config_path.display(),
                why
            ),
        })?;
    
    println!("Finished, the config can be edited at `{}`", &config_path.display());

    Ok(())
}
