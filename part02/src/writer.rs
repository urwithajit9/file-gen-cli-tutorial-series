use std::fs;
use std::io;
use std::path::Path;

/// Write `content` to `file_path`.
///
/// Returns an `io::Error` if the write fails — we let main.rs decide
/// how to handle and display the error. This keeps writer.rs focused
/// on one job: writing bytes to disk.
pub fn write_file(file_path: &Path, content: &str) -> Result<(), io::Error> {
    fs::write(file_path, content)
}