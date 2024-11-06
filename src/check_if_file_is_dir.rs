use std::process::Command;

pub fn file_is_dir(path: &String) -> bool {
    let cmd = format!("test -d {} && echo true", path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        return stdout.trim() == "true";
    }
    return false;
}
