use std::process::Command;

#[cfg(target_os="macos")]
pub fn open(path: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("open {}", path))
        .output()
        .expect("couldn't display gif");
}

#[cfg(target_os="linux")]
pub fn open(path: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("xdg-open {}", path))
        .output()
        .expect("couldn't display gif");
}

// TODO: windows?
