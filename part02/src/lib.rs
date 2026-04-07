// fgen-series/part02/src/lib.rs

pub mod cli;
pub mod template;
pub mod writer;

// Re-export key types for convenience
pub use cli::{Cli, TemplateChoice};
pub use template::{resolve_content, render_template, list_all_templates};
pub use writer::write_file;