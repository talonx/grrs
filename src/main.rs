use std::path::PathBuf;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

/// simple grep
#[derive(Parser)]
struct Args {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    
//    println!("{:?}", args.pattern);
//    println!("{:?}", args.path);

    let file = match File::open(&args.path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening the file {:?}", error),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => if line.contains(&args.pattern) {
                           println!("{}", line);
                        },
            Err(err) => panic!("Failed to read a line {:?}", err),
        }
    }
    
}
