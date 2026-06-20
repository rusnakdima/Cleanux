use std::path::PathBuf;
pub fn test_data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data")
}
pub fn create_temp_file(path: &str) -> std::io::Result<()> {
    std::fs::File::create(path)?;
    Ok(())
}
pub fn create_temp_dir(path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
pub fn cleanup_temp_dir(path: &str) -> std::io::Result<()> {
    if path.starts_with("/tmp/") || path.starts_with("/var/tmp/") {
        let _ = std::fs::remove_dir_all(path);
    }
    Ok(())
}