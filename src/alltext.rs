use std::env;
use std::io;
use std::io::prelude::*;
use std::process;
use std::str;

const ALLTEXT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args = env::args();
    args.next(); // skipping arg[0]

    let mut show_help = false;
    let mut null_delimiter = false;

    if let Some(arg) = args.next() {
        match arg.as_ref() {
            "--null" => {
                null_delimiter = true
            },
            "--help" => {
                show_help = true
            },
            "--version" => {
                println!("alltext {}", ALLTEXT_VERSION);
                process::exit(0);
            },
            _        => {
                println!("unknown argument");
                process::exit(1);
            }
        }
    }

    if show_help {
        println!("{}", ALLTEXT_HELP);

        process::exit(0);
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
            let buffer = String::from_utf8_lossy(&input_raw);

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

static ALLTEXT_HELP: &'static str = "\
alltext - information about text input (including non-printable characters)

options:
  --null     use NUL (\\0) as line delimiter instead of default LF (\\n)
  --version  print version

example:
  printf \"Hello world.\\r\\n\" | alltext\
";
