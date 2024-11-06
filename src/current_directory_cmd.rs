use crate::{check_if_file_is_dir::file_is_dir, file_checker::check_if_file_exists};
use std::process::Command;

pub fn check_and_move_from_current_dir(
    destination: &String,
    files_to_move_as_string: &String,
    files_to_move: &[String],
) -> bool {
    // Check if current directory exists
    let list_to_check_cmd_string = if cfg!(target_os = "windows") {
        "dir"
    } else {
        "ls"
    };
    let list_to_check_cmd = Command::new("sh")
        .arg("-c")
        .arg(list_to_check_cmd_string)
        .output()
        .expect("An issue occured when trying to find the directory");
    let string_to_check = String::from_utf8_lossy(&list_to_check_cmd.stdout);

    let formatted_destination: String = destination
        .chars()
        .skip(1)
        .take(destination.len() - 2)
        .collect();

    //Move files if current directory exists
    if string_to_check
        .trim()
        .contains(&formatted_destination.trim())
    {
        // Check if the file exists
        let file_exists = check_if_file_exists(&formatted_destination.trim(), files_to_move);
        if file_exists {
            return true;
        }
        let platform_cmd = if cfg!(target_os = "windows") {
            "copy"
        } else {
            "cp"
        };
        let is_dir = file_is_dir(files_to_move_as_string);
        let recursive_appendage = if is_dir { "-r" } else { "" };
        let move_to_dir_cmd_string = format!(
            "{} {} {} {}",
            platform_cmd, recursive_appendage, files_to_move_as_string, destination
        );

        let move_to_dir_cmd = Command::new("sh")
            .arg("-c")
            .arg(move_to_dir_cmd_string)
            .output()
            .expect("An issue occured when moving the files");
        if move_to_dir_cmd.status.success() {
            println!("Copied {} to {}\n", files_to_move_as_string, destination);
        } else {
            print!("File not recognized\n");
        }
        return true;
    }
    return false;
}
