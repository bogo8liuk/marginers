use std::env;
use std::fs::{read_to_string, write};
use std::io::{self};
use std::path::Path;
use std::process::exit;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        eprintln!("Error: File '{}' does not exist", file_path);
        exit(1);
    }

    let content = read_to_string(file_path)?;
    let lines: Vec<&str> = content.split('\n').collect();
    let margined_content = reduce_margins_of_lines(lines).join("\n");

    write(file_path, margined_content)?;

    Ok(())
}

fn reduce_margins_of_lines(mut lines: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(lines.len() * 2);
    let mut current_line: String = String::new();

    lines.iter_mut().for_each(|line| {
        current_line.push_str(line);

        while current_line.len() > 80 {
            match split_at_max(current_line.as_str(), 80) {
                None => {
                    eprintln!(
                        "Unreachable code, cannot split line -> {}",
                        current_line.as_str()
                    );
                    exit(-1)
                }
                Some((line1, line2)) => {
                    if line1.len() == 0 {
                        eprintln!("Found line longer than margin and without whitespace, exiting");
                        exit(1)
                    }
                    res.push(line1.to_owned());
                    current_line = line2.to_owned();
                }
            };
        }

        res.push(current_line.clone());
        current_line.clear();
    });

    res
}

fn split_at_max(s: &str, max: usize) -> Option<(&str, &str)> {
    if s.len() <= max {
        return None;
    }

    let mut i = 0;
    let mut last_whitespace_index = 0;
    let mut any_trailing_whitespace = false;
    let mut last_trailing_whitespace_index = 0;
    let mut iter = s.chars();
    loop {
        let c = iter.next()?;

        if i > max {
            break;
        }

        if c.is_whitespace() {
            if !any_trailing_whitespace {
                last_whitespace_index = i;
            }
            last_trailing_whitespace_index = i;
            any_trailing_whitespace = true;
        } else {
            if any_trailing_whitespace {
                last_whitespace_index = last_trailing_whitespace_index;
            }
            any_trailing_whitespace = false;
        }

        i += 1
    }

    s.split_at_checked(last_whitespace_index)
        .map(|(line1, line2)| (line1, line2.trim_start()))
}
