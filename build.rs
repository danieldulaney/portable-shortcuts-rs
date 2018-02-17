#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

const CONFIG_FILENAME: &str = "assets/config.toml";

#[derive(Debug, Deserialize)]
struct Config {
    template: String,
    src_dir: String,
    paths: Vec<String>,
    envs: HashMap<String, String>,
    shortcuts: HashMap<String, Shortcut>,
    terminal: String,
    terminal_flags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Shortcut {
    target: String,
    cli: bool,
    args: Vec<String>,
}

fn main() {
    eprintln!("Loading config {}", CONFIG_FILENAME);
    let config_data = read_file(CONFIG_FILENAME).unwrap();
    let config: Config = toml::from_str(&config_data).unwrap();
    eprintln!("Found config {:?}", config);

    eprintln!("Loading template file {:?}", config.template);
    let template = read_file(&config.template).unwrap();
    eprintln!("Loaded template");

    println!("cargo:rerun-if-changed={}", CONFIG_FILENAME);
    println!("cargo:rerun-if-changed={}", config.template);


    for (name, data) in &config.shortcuts {
        eprintln!("Handling shortcut {}: {:?}", name, data);

        let mut src_path = PathBuf::from(config.src_dir.clone());
        src_path.push(name);
        src_path.set_extension("rs");

        let target = match data.cli {
            false => &data.target,
            true => &config.terminal,
        };

        let path_replacement = config.paths.iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<String>>()
            .join(", ");

        let env_vars_replacement = (&config.envs).into_iter()
            .map(|(k, _)| format!("{:?}", k))
            .collect::<Vec<String>>()
            .join(", ");

        let env_vals_replacement = (&config.envs).into_iter()
            .map(|(_, v)| format!("{:?}", v))
            .collect::<Vec<String>>()
            .join(", ");

        let args_replacement = match data.cli {
            false => (&data.args).into_iter()
                .map(|s| format!("{:?}", s))
                .collect::<Vec<String>>()
                .join(", "),
            true => {
                let mut args = (&config.terminal_flags).into_iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<String>>()
                    .join(", ");

                args.push_str(", ");
                args.push_str(&format!("{:?}", &data.target));
                args.push_str(", ");

                args.push_str(&(&data.args).into_iter()
                                    .map(|s| format!("{:?}", s))
                                    .collect::<Vec<String>>()
                                    .join(", "));

                args
            }
        };

        let source = replace(&template, "%TARGET%", &format!("{:?}", target));
        let source = replace(&source, "%PATHS%", &path_replacement);
        let source = replace(&source, "%ENV_VARS%", &env_vars_replacement);
        let source = replace(&source, "%ENV_VALS%", &env_vals_replacement);
        let source = replace(&source, "%ARGS%", &args_replacement);

        eprintln!("Writing to source file {:?}", src_path);
        let mut src_file = File::create(src_path).unwrap();
        src_file.write(source.as_bytes()).unwrap();
        eprintln!("Successfully wrote source");
    }

    //panic!("Done with build.rs");
}

fn read_file<P: AsRef<Path>>(filename: P) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    return Ok(string);
}

fn replace(original: &str, pattern: &str, replacement: &str) -> String {
    original.replace(pattern, replacement)
}

