//! Executable option parsing and management.
//!
//! Use the `Options::parse()` functions to get the program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```no_run
//! # use ptiong::Options;
//! let opts = Options::parse();
//! println!("{:#?}", opts);
//! ```


use clap::{AppSettings, App, Arg};
use std::path::PathBuf;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The directory containing config files
    ///
    /// Parent directory must exist
    ///
    /// Default: `"$HOME/.pir-8-emu/"`
    pub input: (String, PathBuf),
}


impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("ptiong")
            .version(crate_version!())
            .author(crate_authors!())
            .about("prium trt brbw")
            .setting(AppSettings::ColoredHelp)
            .args(&[Arg::from_usage("<INPUT_FILE> 'File to read'").validator(|s| filesystem_validator("input", false, &s))])
            .get_matches();

        let input = matches.value_of("INPUT_FILE").unwrap();
        Options { input: (input.to_string(), fs::canonicalize(input).unwrap()) }
    }
}


fn filesystem_validator(label: &str, directory: bool, s: &str) -> Result<(), String> {
    fs::canonicalize(s).map_err(|_| format!("{} \"{}\" not found", label, s)).and_then(|f| if f.is_dir() == directory {
        Ok(())
    } else {
        Err(format!("{} \"{}\" not a {}", label, s, if directory { "directory" } else { "file" }))
    })
}
