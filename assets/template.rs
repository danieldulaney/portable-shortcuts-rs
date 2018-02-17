static TARGET: &'static str = %TARGET%;
static PATHS: &'static [&str] = &[%PATHS%];
static ENV_VARS: &'static [&str] = &[%ENV_VARS%];
static ENV_VALS: &'static [&str] = &[%ENV_VALS%];

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn main() {
    println!("This is the template file!");
    let drive_letter = detect_drive_letter();
    println!("Detected drive letter: {:?}", drive_letter);

    println!("Raw target is {:?}", TARGET);
    println!("Corrected target is {:?}", set_drive_letter(TARGET, drive_letter));
    println!("Corrected target if no drive letter is {:?}", set_drive_letter(TARGET, None));

    let foo = "/this/is/a/path";
    println!("{} corrects to {:?} with the drive letter", foo, set_drive_letter(foo, drive_letter));
    println!("{} corrects to {:?} without the drive letter", foo, set_drive_letter(foo, None));
}

fn set_drive_letter(path: &str, drive_letter: Option<char>) -> OsString {
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
            std::path::Prefix::Disk(d) => path.components().collect::<PathBuf>().into_os_string(),
            _ => path_iter.collect::<PathBuf>().into_os_string(),
        },
        _ => path.components().collect::<PathBuf>().into_os_string(),
    }
}

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

