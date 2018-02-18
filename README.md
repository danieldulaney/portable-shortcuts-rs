# portable-shortcuts-rs

Create portable shortcuts on a USB drive.

## Why does this exist?

Sometimes, you have to do development work on a managed enterprise environment.
One method is to install your environment onto a USB drive, then run the programs
off of it. However, on different computers, Windows can assign different drive
letters to a USB drive based on how many drive letters the system already has
in use. This makes it very tricky to use simple shortcuts, though there are
[workarounds](http://sumtips.com/2013/01/create-shortcut-with-relative-path-in-windows.html),
which usually rely on the fact that a path like `\foo\bar` will be assigned the
drive letter of the current working directory.

However, there are two shortcomings with the standard workarounds:
1. [Some programs](https://www.gnu.org/software/emacs/manual/html_node/emacs/Windows-HOME.html) don't handle missing drive letters well
2. This method makes it hard to manage envrionment variables (particularly `PATH`)

This program uses Rust to generate and build executables that can intelligently
configure the correct `PATH` and other environment variables, then launch a program
from a USB drive, regardless of drive letter.

## How to use

1. Clone the project *onto the USB drive you plan on deploying to*
2. Edit `assets/config.toml` as necessary
3. Run `cargo build && cargo build && cargo run --bin portable-shortcuts`. Yes, really. Explanation
   below.

## How it works

Building and deploying is a 3-step process:

1. Build the project. This runs `build.rs`, which parses `assets/config.toml`,
   generating the source for each shortcut and leaving them in `src/bin`. However,
   because Cargo determines build targets *before* running `build.rs`, the generated
   source files don't actually get built.
2. Build the project again. This (probably) doesn't run `build.rs` again, but it
   does build all of the things in `src/bin`.
3. Run the built portable-shortcuts file. This reads `assets/config.toml` again
   and copies the built shortcuts from step 2 into their correct locations.

The template used for each shortcut is in `assets/template.rs`.

