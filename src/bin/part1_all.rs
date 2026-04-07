use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

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

    /// Convert content to uppercase before writing to the file
    #[arg(short, long)]
    uppercase: bool, // Default is false, so it won't convert to uppercase unless specified
}

fn main() {
    let args = Cli::parse();
    let file_path = args.path.join(&args.name);
    let output_dir = file_path.parent().unwrap_or_else(|| Path::new("."));

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create directories");
    }

    if file_path.exists() && !args.overwrite {
        eprintln!(
            " File already exists: {}. Use --overwrite to replace it.",
            file_path.display()
        );
        std::process::exit(1);
    }

    let content_to_write = if args.uppercase {
        args.content.to_uppercase()
    } else {
        args.content
    };

    match fs::write(&file_path, &content_to_write) {
        Ok(_) => {
            println!(" File created: {}", file_path.display());
        }
        Err(e) => {
            eprintln!(" Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
