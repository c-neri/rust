use std::fs::File;
use std::io::Error;

pub fn create_file(name: &str) -> Result<(), Error> {
    File::create(name)?;
    Ok(())
}
