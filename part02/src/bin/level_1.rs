// Just uses the shared library — TOML is already in TemplateChoice & render_template
use fgen_part02::{Cli, resolve_content, write_file};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    
    // Level 1 doesn't support --list-templates, so name is required
    let name = args.name.expect("--name is required");
    let file_path = args.path.join(&name);
    
    let content = resolve_content(&name, args.content, args.template);
    
    match write_file(&file_path, &content) {
        Ok(_) => println!("✅ Created: {}", file_path.display()),
        Err(e) => {
            eprintln!("❌ Failed: {}", e);
            std::process::exit(1);
        }
    }
}