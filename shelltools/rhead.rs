// head -- display first lines of a file
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader, BufRead}; 

fn head(fname: &String, count: Option<usize>, bytes: bool) -> io::Result<()> {
    let input = File::open(fname)?;
    let reader = BufReader::new(input);
    let count = count.unwrap_or(10);

    if bytes {
        for byte in reader.bytes().take(count) {
            print!("{}", byte.unwrap() as char);
        }

    } else {
        for line in reader.lines().take(count) {
            println!("{}", line?);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args.last().unwrap();
    let bytes = *&args.contains(&"-c".to_string());
    let lines = *&args.contains(&"-n".to_string());
    let mut count = None;

    if lines {
        let index = &args.iter().position(|x| x == &"-n".to_string()).unwrap() + 1; 
        let n = *&args.get(index).unwrap().parse::<usize>().unwrap();
        count = Some(n);
    } else if bytes {
        let index = &args.iter().position(|x| x == &"-c".to_string()).unwrap() + 1; 
        let n = *&args.get(index).unwrap().parse::<usize>().unwrap();
        count = Some(n);
    }

    let _ = head(&fname, count, bytes);
}
