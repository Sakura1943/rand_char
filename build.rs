#[path = "./src/cli.rs"]
mod cli;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh}
};
use cli::Cli;
use std::{fs::create_dir, io::Result, path::Path};

fn main() -> Result<()> {
    let cmd = &mut Cli::cmds();
    let bin_name = "rand_char";
    let out_dir = "completions";
    if !Path::new(out_dir).exists() {
        create_dir(out_dir)?;
    }
    generate_to(Bash, cmd, bin_name, out_dir)?;
    generate_to(Fish, cmd, bin_name, out_dir)?;
    generate_to(Zsh, cmd, bin_name, out_dir)?;
    Ok(())
}
