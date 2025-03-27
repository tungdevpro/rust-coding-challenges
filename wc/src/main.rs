use std::{fs::File, io::BufRead};

use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::C { name } => {
            let file_contents = std::fs::metadata(&name);

            if file_contents.is_err() {
                println!("Error reading test text.");
                return;
            }
            println!("{} {}", file_contents.unwrap().len(), name);
        }
        Commands::L { name } => {
            let file = File::open(&name);
            if file.is_err() {
                println!("Erorr open file");
                return;
            }

            let file = file.unwrap();
            let buffered = std::io::BufReader::new(file);
            let line_count = buffered.lines().count();

            println!("{} {}", line_count, name)
        }
        Commands::W { name } => {
            let file = File::open(&name);
            if file.is_err() {
                println!("Erorr open file");
                return;
            }

            let file = file.unwrap();
            let buffered = std::io::BufReader::new(file);
            let mut word_count = 0;

            for line in buffered.lines() {
                if line.is_ok() {
                    let line = line.unwrap();
                    word_count += line.split_whitespace().count();
                }
            }

            println!("{} {}", word_count, name)
        }
        Commands::M { name } => {
            let file = File::open(&name);
            if file.is_err() {
                println!("Erorr open file");
                return;
            }

            let file = file.unwrap();
            let buffered = std::io::BufReader::new(file);
            let mut word_count = 0;

            for line in buffered.lines() {
                if line.is_ok() {
                    word_count += line.unwrap().chars().count();
                }
            }

            println!("{} {}", word_count, name)
        }
    }
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
