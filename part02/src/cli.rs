use clap::Parser;
use std::path::PathBuf;

/// The template to apply to a new file.
///
/// Using an enum here gives us two benefits:
/// 1. clap auto-validates the value — passing `--template blah` is a hard error.
/// 2. We can match on it exhaustively — the compiler tells us if we forget a case.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TemplateChoice {
    /// Plain text, no special formatting
    Plain,
    /// A minimal JSON object skeleton
    Json,
    /// A CSV file with a sample header row
    Csv,
    /// An .env file with commented example variables
    Env,
}

/// fgen — generate files from the command line
#[derive(Parser, Debug)]
#[command(
    name = "fgen",
    version = "0.2.0",
    about = "Generate files with optional templates"
)]
pub struct Cli {
    /// Name of the file to create (e.g. notes.txt, data.csv)
    #[arg(short, long)]
    pub name: String,

    /// Content to write into the file (overrides template content)
    #[arg(short, long)]
    pub content: Option<String>,

    /// Directory where the file will be saved (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Explicitly choose a file template (overrides extension detection)
    #[arg(short, long, value_enum)]
    pub template: Option<TemplateChoice>,
}