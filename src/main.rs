extern crate serde_json;

use std::env;
use std::io::{self, BufRead};
use std::process::{self, Command};

use serde_json::Value as JsonValue;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: i3status-run <program> [<args>]");
        process::exit(1);
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        // we can ignore lines that start with '{' and lines that only
        // have a '[' character;
        let line = if !line.is_empty() && line.get(0..1) != Some("{") && line != "[" {
            let output = Command::new(&args[0])
                .args(&args[1..])
                .output()
                .expect("Failed to run program");
            let output: JsonValue = serde_json::from_slice(&output.stdout)
                .expect("Could not parse program output as JSON");

            // if the first character is a "," then skip that
            let (prefix, line) = if line.get(0..1) == Some(",") {
                (",", &line[1..])
            } else {
                ("", line.as_str())
            };

            let mut json: JsonValue = serde_json::from_str(line).expect(&format!(
                "Could not parse JSON output from i3status: {}",
                line
            ));
            if json.is_array() {
                let mut array = vec![output];
                array.append(json.as_array_mut().unwrap());
                format!(
                    "{}{}",
                    prefix,
                    serde_json::to_string(&array).expect("Could not serialize array to string")
                )
            } else {
                format!("{}{}", prefix, line.to_owned())
            }
        } else {
            line
        };

        println!("{}", line);
    }
}
