#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate clap;
extern crate env_logger;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};

use clap::{App, Arg};

static ABOUT: &str = r#"
Generates an ember-based frontend for your rustdoc documentation.

This is the frontend bundled with rustdoc. See the documentation at
https://github.com/steveklabnik/rustdoc for information on how to use alternative frontends.
"#;

lazy_static! {
    static ref ASSETS: Vec<Asset> = include!(concat!(env!("OUT_DIR"), "/asset.in"));
}

/// Static assets compiled into the binary so we get a single executable. These are dynamically
/// generated with the build script based off of items in the `frontend/dist` folder.
#[derive(Debug)]
pub struct Asset {
    /// Relative path of the file
    pub path: &'static str,

    /// Contents of the file
    pub contents: &'static [u8],
}

impl Asset {
    fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref().join(self.path);
        info!("creating {}", path.display());

        let parent = path.parent().unwrap();
        fs::create_dir_all(parent)?;
        let mut file = File::create(&path)?;
        file.write_all(self.contents)?;

        Ok(())
    }
}

fn run<P: AsRef<Path>>(output_path: P) -> io::Result<()> {
    for asset in ASSETS.iter() {
        asset.create(&output_path)?;
    }

    Ok(())
}

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("rustdoc-ember")
        .about(ABOUT)
        .arg(
            Arg::with_name("output")
                .required(true)
                .takes_value(true)
                .short("o")
                .long("output")
                .help("the directory that the assets should be generated in"),
        )
        .arg(Arg::with_name("data").required(true).help(
            "the documentation data generated by the backend",
        ))
        .get_matches();

    let output_dir = PathBuf::from(matches.value_of("output").unwrap());

    if let Err(e) = run(&output_dir) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
