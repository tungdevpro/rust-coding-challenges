use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{Parser, Subcommand};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::C { name } => match std::fs::metadata(&name) {
            Ok(meta) => println!("{} {}", meta.len(), name),
            Err(_) => println!("error read file"),
        },
        Commands::L { name } => {
            let count_line = count_lines(&name)?;
            println!("{} {}", count_line, name)
        }
        Commands::W { name } => {
            let count_word = count_words(&name)?;
            println!("{} {}", count_word, name)
        }
        Commands::M { name } => {
            let count_chars = count_chars(&name)?;
            println!("{} {}", count_chars, name)
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "wc")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "c", short_flag = 'c')]
    C { name: String },

    #[clap(name = "l", short_flag = 'l')]
    L { name: String },

    #[clap(name = "w", short_flag = 'w')]
    W { name: String },

    #[clap(name = "m", short_flag = 'm')]
    M { name: String },
}

fn open_file(name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(name)?;
    Ok(BufReader::new(file))
}

fn count_lines(name: &str) -> std::io::Result<usize> {
    let file = open_file(name)?;
    Ok(file.lines().count())
}

fn count_words(name: &str) -> std::io::Result<usize> {
    let file = open_file(name)?;
    let mut count = 0;

    for line in file.lines() {
        count += line?.split_whitespace().count();
    }

    Ok(count)
}

fn count_chars(name: &str) -> std::io::Result<usize> {
    let file = open_file(name)?;
    let mut char_count = 0;

    for line in file.lines() {
        char_count += line?.chars().count();
    }

    Ok(char_count)
}
