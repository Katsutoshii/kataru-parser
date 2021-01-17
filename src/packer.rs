use super::{Config, Loadable, Story};
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

fn dump<S: Serialize>(obj: &S, outpath: &Path) -> io::Result<()> {
    let mut outfile = BufWriter::new(File::create(outpath).unwrap());
    outfile.write(&rmp_serde::to_vec(obj).unwrap())?;
    Ok(())
}

/// Parses the config and story files into RMP and writes to the output.
pub fn pack(dir: &str, outdir: &str) -> io::Result<()> {
    let path = Path::new(dir);
    let outpath = Path::new(outdir);

    dump(
        &Story::load(&path.join("passages"))?,
        &outpath.join("story"),
    )?;
    dump(
        &Config::load(&path.join("config.yml"))?,
        &outpath.join("config"),
    )?;
    Ok(())
}

/// Deserializes binary.
pub fn unpack(config_bytes: &[u8], story_bytes: &[u8]) -> (Config, Story) {
    (
        rmp_serde::from_slice(config_bytes).unwrap(),
        rmp_serde::from_slice(story_bytes).unwrap(),
    )
}
