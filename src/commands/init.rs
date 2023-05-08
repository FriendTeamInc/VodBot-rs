use crate::config;
use crate::util;

pub fn run() -> Result<(), util::ExitMsg> {
    let conf = config::Config {
        ..Default::default()
    };

    // println!("{:?}", conf);
    // println!("test {}", conf.directories.vods.display());

    util::create_dir(&conf.directories.vods)?;
    util::create_dir(&conf.directories.highlights)?;
    util::create_dir(&conf.directories.uploads)?;
    util::create_dir(&conf.directories.premieres)?;
    util::create_dir(&conf.directories.clips)?;
    util::create_dir(&conf.directories.temp)?;
    util::create_dir(&conf.directories.stage)?;
    util::create_dir(&conf.directories.thumbnail)?;

    Ok(())
}
