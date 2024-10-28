use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) -> std::io::Result<()> {
    let mut greeting_file = match OpenOptions::new().write(true).create(true).open(file) {
        Ok(file) => file,
        Err(error) =>  panic!("{:?}", error),
    };
  
    greeting_file.write_all(content.as_bytes())?;
    Ok(())
}
