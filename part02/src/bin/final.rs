// Identical to level_3.rs, but this is the "complete" binary users would install
// You can add polish here: better error messages, progress indicators, etc.
use fgen_part02::{Cli, TemplateChoice,resolve_content, write_file, template::list_all_templates};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    
    if args.list_templates {
        println!("🎨 Available templates:\n");
        for (choice, example) in list_all_templates() {
            println!("{:?} → extension: .{}", 
                match choice {
                    TemplateChoice::Plain => "plain",
                    TemplateChoice::Json => "json", 
                    TemplateChoice::Csv => "csv",
                    TemplateChoice::Env => "env",
                    TemplateChoice::Toml => "toml",
                },
                match choice {
                    TemplateChoice::Plain => "txt",
                    TemplateChoice::Json => "json",
                    TemplateChoice::Csv => "csv", 
                    TemplateChoice::Env => "env",
                    TemplateChoice::Toml => "toml",
                }
            );
            println!("  Preview: {}", example.lines().next().unwrap_or(""));
        }
        return;
    }
    
    let name = args.name.expect("--name is required for file generation");
    let file_path = args.path.join(&name);
    let content = resolve_content(&name, args.content, args.template);
    
    match write_file(&file_path, &content) {
        Ok(_) => {
            println!("✅ Successfully created: {}", file_path.display());
            if !content.is_empty() {
                println!("   Preview: {}", content.lines().next().unwrap_or(""));
            }
        }
        Err(e) => {
            eprintln!("❌ Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}