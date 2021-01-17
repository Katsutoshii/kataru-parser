use super::{Config, Parsable, Story};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

/// Reads a file from a given path into new string.
fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub trait Loadable {
    fn load<P: AsRef<Path>>(path: P) -> io::Result<Self>
    where
        Self: Sized;
}

impl Loadable for Story {
    /// Loads a story from a given directory.
    fn load<P: AsRef<Path>>(path: P) -> io::Result<Story> {
        let mut story_str = String::new();
        for path in fs::read_dir(path)? {
            story_str.push_str(&read(&path?.path())?);
        }
        Ok(Story::parse(&story_str).unwrap())
    }
}

impl Loadable for Config {
    /// Loads a config from a given directory.
    fn load<P: AsRef<Path>>(path: P) -> io::Result<Config> {
        let config_str = read(&path)?;
        Ok(Config::parse(&config_str).unwrap())
    }
}
