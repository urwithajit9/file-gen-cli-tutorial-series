use fgen_part02::{Cli, resolve_content, write_file, template::list_all_templates};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    
    // Level 2 feature: --list-templates
    if args.list_templates {
        println!("📋 Available templates:\n");
        for (choice, example) in list_all_templates() {
            println!("{:?}:", choice);
            println!("  Example output:\n{}", 
                example.lines().take(3).collect::<Vec<_>>().join("\n"));
            println!();
        }
        std::process::exit(0);
    }
    
    // Normal flow requires --name
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