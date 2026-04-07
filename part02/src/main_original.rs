// Declare the modules. Rust will look for:
//   mod cli;      → src/cli.rs
//   mod template; → src/template.rs
//   mod writer;   → src/writer.rs
mod cli;
mod template;
mod writer;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    // Build the full output path
    let file_path = args.path.join(&args.name);

    // Resolve what content to write, using the priority chain in template.rs
    let content = template::resolve_content(
        &args.name,
        args.content,
        args.template,
    );

    // Write and handle the result
    match writer::write_file(&file_path, &content) {
        Ok(_) => {
            println!("✅  Created: {}", file_path.display());

            // Show the user which template was used (or that it was custom/empty)
            if content.is_empty() {
                println!("    (empty file)");
            } else {
                // Show just the first line as a preview
                let preview = content.lines().next().unwrap_or("");
                println!("    Preview: {}", preview);
            }
        }
        Err(e) => {
            eprintln!("❌  Failed: {}", e);
            std::process::exit(1);
        }
    }
}