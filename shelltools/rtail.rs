// tail -- display last lines of a file
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader, BufRead}; 
use std::collections::VecDeque;

fn tail(fname: &String, count: Option<usize>, bytes: bool) -> io::Result<()> {
    let input = File::open(fname)?;
    let reader = BufReader::new(input);
    let count = count.unwrap_or(10);
    let mut buf = VecDeque::with_capacity(count);
    let mut lbuf = VecDeque::with_capacity(count);

    if bytes {
        for byte in reader.bytes() {
            if buf.len() == count {
                buf.pop_front(); 
            }
            buf.push_back(byte.unwrap());
        }
        for byte in buf {
            print!("{}", byte as char); 
        }

    } else {
        for line in reader.lines() {
            if lbuf.len() == count {
                lbuf.pop_front();
            }
            lbuf.push_back(line.unwrap());
        }

        for line in lbuf {
            println!("{}", line);
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

    let _ = tail(&fname, count, bytes);
}
