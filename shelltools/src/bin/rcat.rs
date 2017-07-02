extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::{self, BufReader, BufRead}; 

fn cat(fname: &String, number: bool, non_blank: bool) -> io::Result<()> {
    let input = File::open(fname)?;
    let buffer = BufReader::new(input);
    let mut counter = 1;

    for (num, line) in buffer.lines().enumerate() {
        let chars = line?;
        let length = *&chars.len() as u64;

        match number {
            true => println!("{:6} {}", num+1, chars),
            false => {
                match non_blank {
                    true => {
                        if length > 0 {
                            println!("{:6} {}", counter, chars);
                            counter += 1;
                        } else {
                            println!("{:6} {}", ' ', chars);
                        }
                    },
                    false => println!("{}", chars),
                }
            },
        }
    }

    Ok(())
}

fn main() {
    let matches = App::new("rcat")
        .version("0.0.1")
        .author("Matt Gathu <mattgathu@gmail.com>")
        .about("simple cat command implemented in rust")
        .arg(Arg::with_name("n")
             .short("n")
             .help("Number the output lines, starting at 1."))
        .arg(Arg::with_name("b")
             .short("b")
             .help("Number non-blank output lines, starting at 1."))
        .arg(Arg::with_name("file")
             .help("input file")
             .required(true)
             .multiple(true))
        .get_matches();

    let files: Vec<_> = matches.values_of("file").unwrap().collect();
    let number = matches.is_present("n");
    let non_blank = matches.is_present("b");
   
    for file in &files {
        cat(&file.to_string(), number, non_blank).unwrap();
    }
}
