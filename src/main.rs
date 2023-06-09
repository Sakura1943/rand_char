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
        if !cli.disable_save {
            eprintln!(
                "{} the argument '{}' cannot be used without '{}'",
                "error:".bright_red().bold(),
                "--stdout".yellow(),
                "--disable-save".yellow()
            );
            exit(1);
        }

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
    if !cli.stdout {
        println!(
            "Total: {}, single char length: {}\n",
            count.to_string().green(),
            length.to_string().green()
        );
    }
    if !cli.disable_save {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(save_path)?;
        if !cli.stdout {
            for (index, char) in chars.iter().enumerate() {
                println!("{}{char}", format!("{}: ", index + 1).green().bold());
                file.write_all(format!("{char}\n").as_bytes())?;
            }
            println!(
                "\nThe result is saved in `{}`",
                fs::canonicalize(save_path)?.to_string_lossy().green()
            );
        } else {
            for char in chars {
                println!("{char}");
                file.write_all(format!("{char}\n").as_bytes())?;
            }
        }
    } else {
        if !cli.stdout {
            for (index, char) in chars.iter().enumerate() {
                println!("{}{char}", format!("{}: ", index + 1).green().bold());
            }
            return Ok(());
        }
        for char in chars {
            println!("{char}");
        }
    }
    Ok(())
}
