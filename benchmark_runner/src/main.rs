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

        let num_lines_vec = vec![100, 1e3 as i32, 1e4 as i32, 1e5 as i32, 1e6 as i32];
        let chars_per_line_vec = vec![10, 50, 100, 1000];

        println!("#num_lines,chars_per_line,mean_time,errbar");

        for num_lines in &num_lines_vec {
            for chars_per_line in &chars_per_line_vec {

                let mut combined_filegen_command = filegen_command_string.to_owned();
                combined_filegen_command.push_str(" ");
                combined_filegen_command.push_str(csv_file_path);
                combined_filegen_command.push_str(" ");
                combined_filegen_command.push_str(&num_lines.to_string());
                combined_filegen_command.push_str(" ");
                combined_filegen_command.push_str(&chars_per_line.to_string());


                let mut combined_run_command = run_command_string.to_owned();
                combined_run_command.push_str(" ");
                combined_run_command.push_str(csv_file_path);


                // Generate the csv file
                Command::new("bash")
                        .arg("-c")
                        .arg(combined_filegen_command)
                        .output()
                        .expect("Failed to execute filegen process.");

                // Run experiments to measure reading time
                let mut runtimes_seconds = Vec::<f64>::with_capacity(NUMBER_OF_TRIALS as usize);

                let seconds_extraction_regex = Regex::new(r"Took (.*) seconds").unwrap();

                for _ in 0..NUMBER_OF_TRIALS {
                    let output = Command::new("bash")
                                         .arg("-c")
                                         .arg(&combined_run_command)
                                         .output()
                                         .expect("Failed to execute process.");

                    let stdout = output.stdout;
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

                println!("{},{},{},{}", num_lines, chars_per_line, mean, errbar);
            }
        }

    }

}
