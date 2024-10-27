use std::{env::ArgsOs, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let filename = parse_filename(std::env::args_os())?;

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => return Err(Box::new(e)),
    };
    let numbers = contents.lines().map(find_numbers);

    println!("{:#?}", numbers.collect::<Vec<_>>());

    Ok(())
}

fn parse_filename(args: ArgsOs) -> Result<String, Box<dyn error::Error>> {
    let mut args = args.map(|arg| arg.into_string());
    args.next(); //first argument is always command that started this app
    Ok(args
        .next()
        .ok_or("No file specified to process")?
        .or(Err("Invalid filename"))?)
}

fn find_numbers(search: &str) -> Option<u8> {
    let mut chars = search.chars();
    let mut chars_rev = chars.clone().rev();

    let number1: Option<char>;
    let number2: Option<char>;

    if let Some(x) = chars.find(|c| c.is_ascii_digit()) {
        number1 = Some(x);
    } else {
        number1 = None;
    }
    if let Some(x) = chars_rev.find(|c| c.is_ascii_digit()) {
        number2 = Some(x);
    } else {
        number2 = None;
    }

    if number1.is_none() || number2.is_none() {
        None
    } else {
        let mut tempstr = String::new();
        tempstr.push(number1.unwrap());
        tempstr.push(number2.unwrap());
        Some(tempstr.parse().unwrap())
    }
}
