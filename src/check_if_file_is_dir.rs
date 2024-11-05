use std::process::Command;

pub fn file_is_dir(path: &String) -> bool {
    let cmd = format!("test -d {} && echo true", path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    // Check if the command succeeded and print the output
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.trim() == "true";
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
        return false;
    }
}
