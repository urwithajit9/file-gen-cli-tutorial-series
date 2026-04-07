// fgen-series/part02/src/main.rs
mod cli;
mod template;
mod writer;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    // Handle --list-templates early exit (if you added that flag)
    if args.list_templates {
        println!(" Available templates:");
        for choice in [
            cli::TemplateChoice::Plain,
            cli::TemplateChoice::Json,
            cli::TemplateChoice::Csv,
            cli::TemplateChoice::Env,
            cli::TemplateChoice::Toml,
        ] {
            println!("  - {:?}", choice);
        }
        return; // Exit early, no file creation needed
    }

    // Unwrap name safely — clap ensures it's Some when list_templates is false
    let name = args.name.as_ref().expect("--name is required");
    
    // Build the full output path (now name is &String, which implements AsRef<Path>)
    let file_path = args.path.join(name);

    // Resolve content (pass &str, not &Option<String>)
    let content = template::resolve_content(
        name,  // &String coerces to &str
        args.content,
        args.template,
    );

    // Write and handle the result
    match writer::write_file(&file_path, &content) {
        Ok(_) => {
            println!(" Created: {}", file_path.display());
            if content.is_empty() {
                println!("    (empty file)");
            } else {
                let preview = content.lines().next().unwrap_or("");
                println!("    Preview: {}", preview);
            }
        }
        Err(e) => {
            eprintln!(" Failed: {}", e);
            std::process::exit(1);
        }
    }
}