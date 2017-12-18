use std::process::Command;
use std::fs;
use std::path::Path;

/// Opens a file with it's specified application.
#[cfg(target_os="macos")]
pub fn open(path: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("open {}", path))
        .output()
        .expect("couldn't display gif");
}

/// Opens a file with it's specified application.
#[cfg(target_os="linux")]
pub fn open(path: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("xdg-open {}", path))
        .output()
        .expect("couldn't display gif");
}

// TODO: windows?

/// Creates a directory at `path` if one does not already exist there.
/// If there is already a directory there and `wipe` is true, it will attempt to remove that directory first.
/// If a file already exists there, it will probably panic.
pub fn make_dir(path: &str, wipe: bool) {
    if Path::new(path).exists() {
        if wipe {
            wipe_dir(path);
            fs::create_dir(path).unwrap();
        }
    }
    else {
        fs::create_dir(path).unwrap();
    }
}

/// Removes a directory if it exists.
pub fn wipe_dir(path: &str) {
    if Path::new(path).exists() {
        fs::remove_dir_all(path).unwrap();
    }
}
