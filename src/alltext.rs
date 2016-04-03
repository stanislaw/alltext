use std::env;
use std::io;
use std::io::prelude::*;
use std::str;

fn main() {
    let mut args = env::args();
    args.next(); // skipping arg[0]

    let null_delimiter: bool;

    if let Some(x) = args.next() {
        if x == "--null" {
            null_delimiter = true;
        } else {
            null_delimiter = false;
        }
    } else {
        null_delimiter = false;
    }

    let delimiter: u8;
    if null_delimiter {
        delimiter = b'\0';
    } else {
        delimiter = b'\n';
    };

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut input_raw: Vec<u8> = Vec::new();

    while stdin.read_until(delimiter, &mut input_raw).unwrap() > 0 {
        let mut output = String::new();

        if null_delimiter {
            if input_raw.last().is_some() {
                let needs_pop = {
                    let last: &u8 = input_raw.last().unwrap();

                    last == &delimiter
                };

                if needs_pop {
                    input_raw.pop();
                }
            }
        }

        {
            let buffer = str::from_utf8(&input_raw).unwrap();

            for c in buffer.chars() {
                let pp: String = match c {
                    '\x00' => String::from("NUL"),
                    '\x0A' => String::from("LF"),
                    '\x0D' => String::from("CR"),
                    '\x1b' => String::from("ESC"),
                    '\x20' => String::from("Space"),
                    '\x00'...'\x20' => format!("{}", c as i32),
                    _ => c.to_string()
                };

                output.push_str(&pp);
                output.push(' ');
            }
        }

        &input_raw.clear();

        output.pop();

        println!("{}", output);
    }
}

