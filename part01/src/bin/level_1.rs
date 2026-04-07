use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// fgen — a simple file generator CLI
#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.1.0",
    about = "Generate files with custom content from the command line"
)]
struct Cli {
    /// Name of the file to create (e.g. notes.txt, config.json)
    #[arg(short, long)]
    name: String,

    /// Content to write into the file
    #[arg(short, long, default_value = "")]
    content: String,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Overwrite the file if it already exists
    #[arg(short, long)]
    overwrite: bool, // Default is false, so it won't overwrite existing files unless specified
}

fn main() {
    let args = Cli::parse();
    let file_path = args.path.join(&args.name);

    if file_path.exists() && !args.overwrite {
        eprintln!(
            " File already exists: {}. Use --overwrite to replace it.",
            file_path.display()
        );
        std::process::exit(1);
    }

    match fs::write(&file_path, &args.content) {
        Ok(_) => {
            println!("File created: {}", file_path.display());
        }
        Err(e) => {
            eprintln!(" Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
