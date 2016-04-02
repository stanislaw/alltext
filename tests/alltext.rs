// Make-related variables are capitalized:

#![allow(non_snake_case)]

use std::process::{Command, Stdio};
use std::env;
use std::io::prelude::*;

fn run_alltext(input: &str) -> String {
    let current_dir = env::current_dir().unwrap();

    let ALLTEXT_EXEC;
    if let Ok(value) = std::env::var("ALLTEXT_EXEC") {
        ALLTEXT_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    //println!("{}", ALLTEXT_EXEC.display());

    let test_command = format!("{}", ALLTEXT_EXEC.display());

    //println!("{}", test_command);

    let process = Command::new(test_command).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    process.stdin.unwrap().write(input.as_bytes()).unwrap_or_else(|e| {
         panic!("error: {}", e)
    });

    let mut output = String::new();
    process.stdout.unwrap().read_to_string(&mut output).unwrap();

    return output
}

#[test]
fn it_echoes_string_back_with_spaces() {
    let input = "Hello!";
    let expected_output = "H e l l o !\n";

    let output = run_alltext(input);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_spaces_printed_as_Space() {
    let input = "A B C";
    let expected_output = "A Space B Space C\n";

    let output = run_alltext(input);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_escape_as_ESC() {
    let input = "\x1b\x1b\x1b";
    let expected_output = "ESC ESC ESC\n";

    let output = run_alltext(input);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_10_as_LF() {
    let input = "\x0a\x0a\x0a";
    let expected_output = "LF\nLF\nLF\n";

    let output = run_alltext(input);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_13_as_CR() {
    let input = "\x0d\x0d\x0d";
    let expected_output = "CR CR CR\n";

    let output = run_alltext(input);

    assert_eq!(output, expected_output);
}

#[test]
	fn it_echoes_3_as_digit_3() {
	let input = "\x03\x03\x03";
	let expected_output = "3 3 3\n";

	let output = run_alltext(input);

	assert_eq!(output, expected_output);
}

