use crate::config;

pub fn run() {
    let conf = config::Config {
        ..Default::default()
    };

    println!("{:?}", conf);
    println!("test {}", conf.directories.vods.display())
}
