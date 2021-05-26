use structopt::StructOpt;

mod config;
mod migrate_dir;
mod mount_dir;

use config::Config;
use migrate_dir::MigrateDir;
use mount_dir::MountDir;

fn do_program(config: Config) {
    if config.migrate {
        let migrate_command = MigrateDir::from(config);
        migrate_command.do_migrate()
    } else {
        let virtualize_command = MountDir::from(config);
        virtualize_command.do_virtualize()
    }
}

fn main() {
    do_program(Config::from_args());
}
