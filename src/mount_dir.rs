use std::path::PathBuf;

use crate::config::Config;

#[derive(Debug)]
pub struct MountDir {
    folder: PathBuf,
    debug: bool,
}

impl MountDir {
    pub fn new(folder: PathBuf, debug: bool) -> Self {
        Self { folder, debug }
    }

    pub fn do_virtualize(self) {
        println!("Would Mount the dir: {:?}", self.folder)
    }
}

impl From<Config> for MountDir {
    fn from(config: Config) -> Self {
        MountDir::new(config.folder, config.debug)
    }
}
