use std::path::PathBuf;

use clap::{Command, CommandFactory, Parser};

#[derive(Debug, Parser, Clone)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// The length of the generated single string
    #[arg(short, long, default_value_t = 10)]
    pub length: u8,
    /// Number of strings generated
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
    /// Ignore dangerous words
    #[arg(short, long, conflicts_with = "ignore_symbol")]
    pub ignore: bool,
    /// Ignore Symbol
    #[arg(long, conflicts_with = "ignore")]
    pub ignore_symbol: bool,
    /// The path where the result will be saved
    #[arg(short, long, value_name = "SAVE_PATH", default_value = "result.txt")]
    pub save: PathBuf,
    /// Only Number(integer)
    #[arg(
        long,
        conflicts_with_all(["only_lowercase", "only_letter", "only_uppercase"])
    )]
    pub only_number: bool,
    /// Only letter
    #[arg(
        long,
        conflicts_with_all(["only_number", "only_uppercase", "only_lowercase"])
    )]
    pub only_letter: bool,
    /// Only letters in upper case
    #[arg(long, conflicts_with_all(["only_number", "only_letter", "only_lowercase"]))]
    pub only_uppercase: bool,
    /// Only letters in lower case
    #[arg(long, conflicts_with_all(["only_number", "only_letter", "only_uppercase"]))]
    pub only_lowercase: bool,
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
