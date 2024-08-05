use std::fs;
use std::io;
use std::path::Path;

/// Creates a directory and all of its parent components if they are missing.
/// This is similar to `fs::create_dir_all`, but provides a more descriptive error message.
///
/// # Arguments
///
/// * `path` - The path to the directory to create.
///
/// # Returns
///
/// * `Result<(), io::Error>` - An empty result on success, or an io::Error on failure.
pub fn create_dir_all_safe<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to create directory {}: {}", path.as_ref().display(), e),
        )
    })
}
