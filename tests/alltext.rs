#![allow(non_snake_case)]

const ALLTEXT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::process::{Command, Stdio};
use std::env;
use std::io::prelude::*;

fn run_alltext(input: &str, args: Vec<&str>) -> String {
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

    let process = Command::new(test_command).args(&args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap_or_else(|e| {
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

    let args: Vec<&str> = vec![];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_spaces_printed_as_Space() {
    let input = "A B C";
    let expected_output = "A Space B Space C\n";

    let args: Vec<&str> = vec![];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_escape_as_ESC() {
    let input = "\x1b\x1b\x1b";
    let expected_output = "ESC ESC ESC\n";

    let args: Vec<&str> = vec![];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

#[test]
// Special case, because LF is default delimiter
fn it_echoes_10_as_LF() {
    let input = "\x0a\x0a\x0a";
    let expected_output = "LF\nLF\nLF\n";

    let args: Vec<&str> = vec![];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

#[test]
fn it_echoes_13_as_CR() {
    let input = "\x0d\x0d\x0d";
    let expected_output = "CR CR CR\n";

    let args: Vec<&str> = vec![];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

#[test]
	fn it_echoes_3_as_digit_3() {
	let input = "\x03\x03\x03";
	let expected_output = "3 3 3\n";

    let args: Vec<&str> = vec![];
    let output = run_alltext(input, args);

	assert_eq!(output, expected_output);
}

// --null parameter tests

#[test]
fn when_null_parameter_it_echoes_10_as_LF() {
    let input = "\x0a\x0a\x0a\0";
    let expected_output = "LF LF LF\n";

    let args: Vec<&str> = vec!["--null"];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

#[test]
fn when_null_parameter_it_echoes_10_as_LF_and_does_not_cut_symbols() {
    let input = "\x0a\x0a\x0a";
    let expected_output = "LF LF LF\n";

    let args: Vec<&str> = vec!["--null"];
    let output = run_alltext(input, args);

    assert_eq!(output, expected_output);
}

// --help parameter

#[test]
fn when_help_parameter_it_prints_help() {
    let args: Vec<&str> = vec!["--help"];
    let current_dir = env::current_dir().unwrap();

    let ALLTEXT_EXEC;
        if let Ok(value) = std::env::var("ALLTEXT_EXEC") {
        ALLTEXT_EXEC = current_dir.join(value);
    } else {
        panic!("Didn't get executable to run");
    }

    let test_command = format!("{}", ALLTEXT_EXEC.display());

    let process = Command::new(test_command).args(&args).stdout(Stdio::piped()).spawn().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    let mut output = String::new();
    process.stdout.unwrap().read_to_string(&mut output).unwrap();

    assert!(output.contains("alltext - information about text input (including non-printable characters)"));
}

// --version parameter

#[test]
fn when_help_parameter_it_prints_version() {
    let args: Vec<&str> = vec!["--version"];
    let current_dir = env::current_dir().unwrap();

    let ALLTEXT_EXEC;
    if let Ok(value) = std::env::var("ALLTEXT_EXEC") {
    ALLTEXT_EXEC = current_dir.join(value);
    } else {
    panic!("Didn't get executable to run");
    }

    let test_command = format!("{}", ALLTEXT_EXEC.display());

    let process = Command::new(test_command).args(&args).stdout(Stdio::piped()).spawn().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
    });

    let mut output = String::new();
    process.stdout.unwrap().read_to_string(&mut output).unwrap();

    let expected_output = format!("alltext {}", ALLTEXT_VERSION);
    assert!(output.contains(&expected_output));
}
