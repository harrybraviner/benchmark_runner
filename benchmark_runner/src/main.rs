extern crate regex;

use std::process::Command;
use std::env;
use std::str;
use regex::Regex;

static NUMBER_OF_TRIALS: u8 = 1;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: cargo run [command to run filegen] [command to run reading] [path to store file]")
    } else {
        let filegen_command_string = &args[1];
        let run_command_string = &args[2];
        let csv_file_path = &args[3];

        let mut combined_run_command = run_command_string.to_owned();
        combined_run_command.push_str(" ");
        combined_run_command.push_str(csv_file_path);


        let runtime_seconds = Vec::<f64>::with_capacity(NUMBER_OF_TRIALS as usize);

        let seconds_extraction_regex = Regex::new(r"Took (.*) seconds").unwrap();

        for _ in 0..NUMBER_OF_TRIALS {
            let output = Command::new("bash")
                                 .arg(&combined_run_command)
                                 .arg(csv_file_path)
                                 .output()
                                 .expect("Failed to execute process.");

            let stdout: Vec<u8> = output.stdout;
            println!("{}", str::from_utf8(&stdout).unwrap());
            let stderr: Vec<u8> = output.stderr;
            println!("{}", str::from_utf8(&stderr).unwrap());
            seconds_extraction_regex.is_match(str::from_utf8(&stdout).unwrap());

        }
    }

}
