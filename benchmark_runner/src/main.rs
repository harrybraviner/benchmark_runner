extern crate regex;

use std::process::Command;
use std::env;
use std::str;
use regex::Regex;

static NUMBER_OF_TRIALS: u8 = 3;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: cargo run [command to run filegen] [command to run reading] [path to store csv file]")
    } else {
        let filegen_command_string = &args[1];
        let run_command_string = &args[2];
        let csv_file_path = &args[3];

        let mut combined_run_command = run_command_string.to_owned();
        combined_run_command.push_str(" ");
        combined_run_command.push_str(csv_file_path);


        let mut runtimes_seconds = Vec::<f64>::with_capacity(NUMBER_OF_TRIALS as usize);

        let seconds_extraction_regex = Regex::new(r"Took (.*) seconds").unwrap();

        for _ in 0..NUMBER_OF_TRIALS {
            let output = Command::new("bash")
                                 .arg("-c")
                                 .arg(&combined_run_command)
                                 .output()
                                 .expect("Failed to execute process.");

            let stdout: Vec<u8> = output.stdout;
            println!("{}", str::from_utf8(&stdout).unwrap());
            let stderr: Vec<u8> = output.stderr;
            println!("{}", str::from_utf8(&stderr).unwrap());
            let cap = match seconds_extraction_regex.captures(str::from_utf8(&stdout).unwrap()) {
                Some(c) => c,
                None => panic!("Output of command did not include a time!"),
            };

            let time_seconds = &cap[1].parse::<f64>().unwrap();

            runtimes_seconds.push(*time_seconds);
        }

        let mut mean = 0.0f64;
        for t in &runtimes_seconds {
            mean += t;
        }
        mean /= NUMBER_OF_TRIALS as f64;

        let mut variance = 0.0f64;
        for t in &runtimes_seconds {
            variance += (t - mean) * (t - mean);
        }
        variance /= NUMBER_OF_TRIALS as f64;
        let stddev = variance.sqrt();
        let errbar = stddev / (NUMBER_OF_TRIALS as f64).sqrt();

        println!("{} +/- {}", mean, errbar);

    }

}
