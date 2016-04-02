// static ALLTEXT_VERSION: &'static str = "0.0.1";
// static ALLTEXT_DATE:    &'static str = "2016-04-02";

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).unwrap() > 0 {
        let mut output = String::new();

        for c in buffer.chars() {

            // http://www.asciitable.com/
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

        output.pop();

        println!("{}", output);

        buffer.clear();
    }
}

/*
    //let mut input_raw = Vec::new();
let mut buffer = String::new();

while stdin.read_line(&mut buffer).unwrap() > 0 {
// work with buffer
println!("{:?}", buffer);

//stdin.lock().read_to_end(&mut input_raw);
//let input = str::from_utf8(&input_raw).unwrap();

for c in buffer.chars() {

// http://www.asciitable.com/
let pp: String = match c {
'\x00' => String::from("NUL"),
'\x0A' => String::from("LF"),
'\x0D' => String::from("CR"),
'\x1b' => String::from("ESC"),
'\x20' => String::from("Space"),
_ => c.to_string(),
};

output.push_str(&pp);
output.push(' ');
}

output.pop();

println!("{}", output);
buffer.clear();
*/
