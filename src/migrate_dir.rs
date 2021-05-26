use std::path::PathBuf;

use crate::{config::Config, mount_dir::MountDir};

#[derive(Debug)]
pub struct MigrateDir {
    virtualize_command: MountDir,
    debug: bool,
    folder: PathBuf,
}

impl MigrateDir {
    pub fn new(folder: PathBuf, debug: bool) -> Self {
        let virtualize_command = MountDir::new(folder.clone(), debug);
        Self {
            virtualize_command,
            debug,
            folder,
        }
    }

    pub fn do_migrate(self) {
        println!("Would Migrate the dir: {:?}", self.folder);
        self.virtualize_command.do_virtualize()
    }
}

impl From<Config> for MigrateDir {
    fn from(config: Config) -> Self {
        MigrateDir::new(config.folder, config.debug)
    }
}
