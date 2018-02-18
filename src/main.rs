#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::env;

const CONFIG_FILENAME: &str = "assets/config.toml";

#[derive(Debug, Deserialize)]
struct Config {
    shortcuts: HashMap<String, Shortcut>,
}

#[derive(Debug, Deserialize)]
struct Shortcut {
    location: String,
}

fn main() {
    println!("This is the main.rs file, DON'T USE AS A SHORTCUT");

    eprintln!("Loading config {}", CONFIG_FILENAME);
    let config_data = read_file(CONFIG_FILENAME).unwrap();
    let config: Config = toml::from_str(&config_data).unwrap();

    let mut out_dir = env::current_exe().expect("could not get current path");
    out_dir.pop();

    println!("Output directory {:?}", out_dir);

    for (name, data) in config.shortcuts {
        println!("Handling {} with {:?}", name, data);

        // Determine built file name
        let mut exe_file = out_dir.clone();
        exe_file.push(&name);
        exe_file.set_extension("exe");

        println!("Copying from {:?} to {:?}", exe_file, data.location);

        // Copy built file to final location
        match fs::copy(&exe_file, &data.location) {
            Ok(size) => println!("Successfully copied {} bytes for {} to {}", size, name, data.location),
            Err(e) => println!("Could not copy {} to {}: {}", name, data.location, e),
        }
    }
}

fn read_file<P: AsRef<Path>>(filename: P) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    return Ok(string);
}

