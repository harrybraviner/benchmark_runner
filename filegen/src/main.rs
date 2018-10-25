extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use rand::Rng;

fn main() {
        
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: cargo run [output filename] [number of lines] [line length]");
    } else {
        let output_filename = &args[1];
        let number_of_lines = args[2].parse::<usize>().unwrap();
        let line_length = args[3].parse::<usize>().unwrap();

        let path = Path::new(output_filename);
        let mut file: File = match File::create(&path) {
            Err(why) => panic!("Couldn't open for writing {}: {}", path.display(), why.description()),
            Ok(file) => file,
        };
        
        let mut rng = rand::thread_rng();

        for _ in 0..number_of_lines {
            // Generate some random data
            let mut this_line_utf8: Vec<u8> = (0..line_length).map(|_| rng.gen_range(65, 91)).collect();
            this_line_utf8.push(0x0au8);

            match file.write_all(&this_line_utf8[..]) {
                Err(why) => panic!("Couldn't write to {}: {}", path.display(), why.description()),
                Ok(_) => {},
            }
        }
        
    }

}
