use std::{fs, path};

fn main() -> Result<(), anyhow::Error> {
    let dirs = fs::read_dir(path::Path::new("/home/jan/workspace/github/dashjockey/src"))
        .map_err(anyhow::Error::msg)?;
    for dir in dirs {
        println!("{:?}", dir?.file_name())
    }
    Ok(())
}
