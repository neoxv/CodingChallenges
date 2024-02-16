use atty::Stream;
use clap::Parser;
use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::str::{self, Utf8Error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        default_value = "default",
        value_name = "Option",
        index = 1,
        required = false
    )]
    option: String,
    #[arg(value_name = "Path", required = false, index = 2)]
    path: Option<String>,
}
fn main() {
    let is_piped_input = !atty::is(Stream::Stdin);
    let parsed_args = Cli::parse();

    if !parsed_args.path.is_none() && is_piped_input {
        eprintln!("Error: Both piped input and file path provided");
        return;
    }

    let args = if parsed_args.path.is_none() && !is_piped_input {
        Cli::parse_from(vec!["ccwc", "default", &parsed_args.option])
    } else {
        parsed_args
    };


    let file_path;
    let file_name;

    if args.path.is_none() {
        file_path = Path::new("");
    } else if let Some(path) = &args.path {
        file_path = Path::new(path);
    } else {
        eprintln!("No file path provided");
        return;
    }

    file_name = if !is_piped_input {
        get_file_name(file_path)
    } else {
        " ".to_string()
    };
    
    process_file(file_path, &args.option, file_name);
}

fn process_file(file_path: &Path, option: &str, name: String) {
    match option {
        "-c" => display_file_byte_count(file_path, name),
        "-l" => display_file_line_count(file_path, name),
        "-w" => display_file_word_count(file_path, name),
        "-m" => display_file_character_count(file_path, name),
        "default" => {
            let byte_len = get_number_of_bytes(file_path);
            let line_len = get_number_of_lines(file_path);
            let word_len = get_number_of_words(file_path);

            match (byte_len, line_len, word_len) {
                (Ok(byte_len), Ok(line_len), Ok(word_len)) => {
                    println!("{} {} {} {}", line_len, word_len, byte_len, name)
                }
                (Err(why_byte), Err(why_line), Err(why_len)) => eprintln!(
                    "couldn't get file information: {}, {}, {}",
                    why_byte, why_line, why_len
                ),
                (Ok(_), Ok(_), Err(why_len)) => {
                    eprintln!("couldn't get file information: {}", why_len)
                }
                (Ok(_), Err(why_line), Ok(_)) => {
                    eprintln!("couldn't get file information: {}", why_line)
                }
                (Ok(_), Err(why_line), Err(why_len)) => {
                    eprintln!("couldn't get file information: {}, {}", why_line, why_len)
                }
                (Err(why_byte), Ok(_), Ok(_)) => {
                    eprintln!("couldn't get file information: {}", why_byte)
                }
                (Err(why_byte), Ok(_), Err(why_len)) => {
                    eprintln!("couldn't get file information: {}, {}", why_byte, why_len)
                }
                (Err(why_byte), Err(why_line), Ok(_)) => {
                    eprintln!("couldn't get file information: {}, {}", why_byte, why_line)
                }
            }
        }
        _ => eprintln!("Invalid option"),
    }
}

fn get_buffer_reader(file_path: &Path) -> std::io::Result<Box<dyn BufRead>> {

    if !atty::is(Stream::Stdin) {
        return Ok(Box::new(BufReader::new(io::stdin())));
    }

    let file = fs::File::open(&file_path)?;
    Ok(Box::new(BufReader::new(file)))
}

fn display_file_byte_count(file_path: &Path, name: String) {
    let len = get_number_of_bytes(file_path);

    match len {
        Ok(len) => println!("{} {}", len, name),
        Err(why) => eprintln!("couldn't get file size: {}", why),
    }
}

fn get_number_of_bytes(file_path: &Path) -> std::io::Result<u64> {
    let metadata = match fs::metadata(&file_path) {
        Ok(metadata) => metadata,
        Err(why) => {
            eprintln!("couldn't get metadata: {}", why);
            return Ok(0);
        }
    };

    Ok(metadata.len())
}

fn get_file_name(file_path: &Path) -> String {
    match file_path.file_name() {
        Some(file_name_osstr) => match file_name_osstr.to_str() {
            Some(file_name) => file_name.to_owned(),
            None => {
                eprintln!("Couldn't convert file name to UTF-8 string.");
                "-".to_owned()
            }
        },
        None => {
            eprintln!("Couldn't extract file name.");
            "-".to_owned()
        }
    }
}

fn display_file_line_count(file_path: &Path, name: String) {
    let len = get_number_of_lines(file_path);

    match len {
        Ok(len) => println!("{} {}", len, name),
        Err(why) => eprintln!("couldn't get file size: {}", why),
    }
}

fn get_number_of_lines(file_path: &Path) -> std::io::Result<u64> {
    let reader = match get_buffer_reader(file_path) {
        Ok(reader) => reader,
        Err(why) => {
            eprintln!("couldn't open {}: {}", file_path.display(), why);
            return Ok(0);
        }
    };

    let lines = reader.lines().count() as u64;
    Ok(lines)
}

fn display_file_word_count(file_path: &Path, name: String) {
    let len = get_number_of_words(file_path);

    match len {
        Ok(len) => println!("{} {}", len, name),
        Err(why) => eprintln!("couldn't get file size: {}", why),
    }
}

fn get_number_of_words(file_path: &Path) -> std::io::Result<usize> {
    let reader = match get_buffer_reader(file_path) {
        Ok(reader) => reader,
        Err(why) => {
            eprintln!("couldn't open {}: {}", file_path.display(), why);
            return Ok(0);
        }
    };

    let mut words = 0;
    for line in reader.lines() {
        let line = line?;
        words += line.split_whitespace().count();
    }

    Ok(words)
}

fn display_file_character_count(file_path: &Path, name: String) {
    let len = get_number_of_characters(file_path);

    match len {
        Ok(len) => println!("{} {} ", len, name),
        Err(why) => eprintln!("couldn't get file size: {}", why),
    }
}

fn get_number_of_characters(file_path: &Path) -> std::io::Result<usize> {
    let mut reader = match get_buffer_reader(file_path) {
        Ok(reader) => reader,
        Err(why) => {
            eprintln!("couldn't open {}: {}", file_path.display(), why);
            return Ok(0);
        }
    };

    let mut buffer = vec![0; 4096]; // Buffer for reading raw bytes
    let mut total_count = 0usize;
    let mut carry_over = Vec::new(); // To handle carry over bytes from partial reads

    loop {
        let bytes_read = if carry_over.is_empty() {
            reader.read(&mut buffer)?
        } else {
            let carry_over_len = carry_over.len();
            buffer[..carry_over_len].copy_from_slice(&carry_over);
            carry_over_len + reader.read(&mut buffer[carry_over_len..])?
        };

        if bytes_read == 0 {
            break; // End of file reached
        }

        match validate_utf8(&buffer[..bytes_read]) {
            Ok(valid_str) => {
                total_count += valid_str.chars().count();
                carry_over.clear();
            }
            Err((valid_up_to, incomplete_bytes)) => {
                total_count += valid_up_to.chars().count();
                carry_over = buffer[bytes_read - incomplete_bytes..bytes_read].to_vec();
            }
        }
    }

    Ok(total_count)
}

/// Validates UTF-8 and returns either the valid string and the number of bytes to carry over
fn validate_utf8(buf: &[u8]) -> Result<&str, (&str, usize)> {
    match std::str::from_utf8(buf) {
        Ok(valid_str) => Ok(valid_str),
        Err(e) => handle_incomplete_utf8(buf, e),
    }
}

/// Handles incomplete UTF-8 sequences by finding the valid string and the number of bytes to carry over
fn handle_incomplete_utf8(buf: &[u8], e: Utf8Error) -> Result<&str, (&str, usize)> {
    let valid_up_to = e.valid_up_to();
    let valid_str = unsafe { std::str::from_utf8_unchecked(&buf[..valid_up_to]) };
    let incomplete_bytes = buf.len() - valid_up_to;
    Err((valid_str, incomplete_bytes))
}
