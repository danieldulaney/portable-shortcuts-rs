#![windows_subsystem = "windows"]

static TARGET: &'static str = %TARGET%;
static PATHS: &'static [&str] = &[%PATHS%];
static ARGS: &'static [&str] = &[%ARGS%];
static ENV_VARS: &'static [&str] = &[%ENV_VARS%];
static ENV_VALS: &'static [&str] = &[%ENV_VALS%];

static PATH_ENTRY_SEP: &'static str = ";";

use std::env;
use std::ffi::OsString;
use std::process::Command;

fn main() {
    let drive_letter = detect_drive_letter();

    // Create a new command with the correct target
    let mut command = Command::new(set_path(TARGET, drive_letter));

    // Set the environment variables
    command.envs(ENV_VARS.iter().zip(ENV_VALS.iter().map(|s| set_path(s, drive_letter))));

    // Get the current PATH variable
    let old_path = match env::var_os("PATH") {
        Some(s) => s,
        None => OsString::new(),
    };

    // Determine what the new PATH variable should be
    let mut path = OsString::new();
    for path_entry in PATHS {
        path.push(set_path(path_entry, drive_letter));
        path.push(PATH_ENTRY_SEP);
    }
    path.push(old_path);

    // Actually set the PATH
    command.env("PATH", path);
    
    // Add command-line arguments
    command.args(ARGS.iter().map(|a| set_path(a, drive_letter)));

    // Panic on launch failure
    match command.spawn() {
        Err(_) => panic!("failed to launch command"),
        Ok(_) => (),
    }

}

fn set_path(path: &str, drive_letter: Option<char>) -> OsString {
    let mut path_iter = path.chars();

    match (drive_letter, path_iter.next(), path_iter.next(), path_iter.next()) {
        (Some(ndl), Some(odl), Some(':'), Some(sep)) if odl == 'a' && (sep == '/' || sep == '\\') => {
            let mut new_path = path.chars().collect::<Vec<char>>();
            new_path[0] = ndl;
            OsString::from(new_path.iter().cloned().collect::<String>())
        },
        _ => OsString::from(path),
    }
}

/*
fn set_path(path: &str, drive_letter: Option<char>) -> OsString {
    let path = Path::new(path);
    let mut path_iter = path.components();
    let first_item = match path_iter.next() {
        None => return OsString::from(""),
        Some(i) => i,
    };

    match first_item {
        std::path::Component::Prefix(p) => match p.kind() {
            std::path::Prefix::Disk(d) if d as char == 'a' || d as char == 'A' => match drive_letter {
                None => path_iter.collect::<PathBuf>().into_os_string(),
                Some(letter) => {
                    let mut final_path = PathBuf::new();
                    final_path.push([letter, ':'].iter().collect::<String>());
                    for item in path_iter {final_path.push(item)}
                    final_path.into_os_string()
                },
            },
            std::path::Prefix::Disk(_) => path.components().collect::<PathBuf>().into_os_string(),
            _ => path_iter.collect::<PathBuf>().into_os_string(),
        },
        _ => path.components().collect::<PathBuf>().into_os_string(),
    }
}
*/
fn detect_drive_letter() -> Option<char> {
    let path = match env::current_exe() {
        Err(_) => return None,
        Ok(path) => path,
    };

    match path.components().next() {
        Some(std::path::Component::Prefix(p)) => match p.kind() {
            std::path::Prefix::Disk(letter) => match std::char::from_u32(letter as u32) {
                Some(c) if c.is_alphabetic() => Some(c),
                Some(_) => None,
                None => None,
            }
            _ => None,
        },
        _ => None,
    }
}

