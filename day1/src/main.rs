use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut args = std::env::args_os().map(|arg| arg.into_string());
    args.next(); //first argument is always command that started this app
    let filename = args
        .next()
        .ok_or("No file specified to process")?
        .or(Err("Invalid filename"))?;

    println!("{}", fs::exists(&filename).unwrap());

    Ok(())
}
