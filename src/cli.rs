use std::path::PathBuf;

use clap::{Command, CommandFactory, Parser};

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// The length of the generated single string
    #[arg(short, long, default_value_t = 10)]
    pub length: u8,
    /// Number of strings generated
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
    /// Ignore dangerous words
    #[arg(short, long)]
    pub ignore: bool,
    /// The path where the result will be saved
    #[arg(short, long, value_name = "SAVE_PATH", default_value = "result.txt")]
    pub save: PathBuf
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
    pub fn cmds() -> Command {
        Self::command()
    }
}
