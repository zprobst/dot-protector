use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Dot Protector",
    about = "A way of protecting your dotfiles with a yubikey"
)]
pub struct Config {
    #[structopt(short, long)]
    /// Print extra debug information.
    pub debug: bool,

    #[structopt(parse(from_os_str))]
    /// The folder to mount or migrate.
    pub folder: PathBuf,

    #[structopt(short, long)]
    /// Migrates an unencrpyted folder to an encrypted volume managed through dot-protector.
    pub migrate: bool,
}
