#[cfg(all(feature = "on-disk-cache", not(windows)))]
mod tests {
    use assert_cmd::prelude::*;
    use std::{fs, path::Path, process::Command};
    use tempfile::tempdir;

    #[test]
    fn test_purge() {
        // Create a mock cache directory
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("cargo-unmaintained/v2");
        fs::create_dir_all(&cache_path).unwrap();
        
        // Create a dummy file inside
        let test_file = cache_path.join("test.txt");
        fs::write(&test_file, "test").unwrap();
        
        // Verify the file exists
        assert!(test_file.exists());
        
        // Run the purge command with environment variable to override cache path
        let mut cmd = Command::cargo_bin("cargo-unmaintained").unwrap();
        
        // Set environment variable for XDG_CACHE_HOME to our temp directory
        cmd.env("XDG_CACHE_HOME", dir.path());
        
        // Run the unmaintained command with --purge
        cmd.arg("unmaintained")
           .arg("--purge");
        
        // Execute and assert success
        cmd.assert().success();
        
        // Verify the directory was removed
        assert!(!cache_path.exists());
    }
} 