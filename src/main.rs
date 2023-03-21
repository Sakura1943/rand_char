use std::{
    fs::{self, OpenOptions},
    io::Write,
    process::exit,
};

use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use rand_char::{cli::Cli, generate::gen_char};

fn main() -> Result<()> {
    let cli = Cli::default();
    let length = cli.length;
    let count = cli.count;
    let save_path = &cli.save;

    if save_path.exists() && !cli.disable_save {
        eprintln!(
            "{} Result `{}` is exists!\n",
            "ERROR:".red(),
            fs::canonicalize(save_path)?.to_string_lossy().yellow()
        );
        let choice = vec!["Append", "New", "Exit"];
        let select_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Text append or create a new file?")
            .items(&choice)
            .default(0)
            .interact()?;
        match select_index {
            1 => fs::remove_file(save_path)?,
            2 => exit(1),
            _ => {}
        }
    }

    let chars = gen_char(cli.clone())?;
    println!(
        "Total: {}, single char length: {}\n",
        count.to_string().green(),
        length.to_string().green()
    );
    if !cli.disable_save {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(save_path)?;

        for (index, char) in chars.iter().enumerate() {
            println!("{}{char}", format!("{}: ", index + 1).green().bold());
            file.write_all(format!("{char}\n").as_bytes())?;
        }
        println!(
            "\nThe result is saved in `{}`",
            fs::canonicalize(save_path)?.to_string_lossy().green()
        );
    } else {
        for (index, char) in chars.iter().enumerate() {
            println!("{}{char}", format!("{}: ", index + 1).green().bold());
        }
    }
    Ok(())
}
