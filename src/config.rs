use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Dot Protector",
    about = "A way of protecting your dotfiles with a yubikey"
)]
pub struct Config {
    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(parse(from_os_str))]
    pub folder: PathBuf,

    #[structopt(short, long)]
    pub migrate: bool,
}
