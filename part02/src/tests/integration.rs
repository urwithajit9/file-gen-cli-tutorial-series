// fgen-series/part02/tests/integration.rs
use std::fs;
use std::path::PathBuf;
use fgen_part02::{resolve_content, write_file, TemplateChoice};

#[test]
fn test_write_toml_file_to_temp_dir() {
    // Get a temporary directory
    let temp_dir = std::env::temp_dir().join("fgen_test");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    
    // Generate content using the shared logic
    let content = resolve_content("config.toml", None, Some(TemplateChoice::Toml));
    
    // Write to a real file
    let file_path = temp_dir.join("test_config.toml");
    write_file(&file_path, &content).expect("Failed to write file");
    
    // Verify the file exists and has expected content
    let written = fs::read_to_string(&file_path).expect("Failed to read file");
    assert!(written.contains("[section]"));
    assert!(written.contains("key = \"value\""));
    
    // Cleanup
    fs::remove_file(&file_path).ok();
    fs::remove_dir(&temp_dir).ok();
}