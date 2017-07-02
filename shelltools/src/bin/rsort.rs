use std::env;
use std::process;
use std::fs::File;
use std::cmp::Ordering;
use std::io::{BufReader, BufRead};


const HELP: &'static str = r#"
NAME
       rsort - sort lines of text files

SYNOPSIS
       rsort [OPTION]... [FILE]...

DESCRIPTION
       Write sorted concatenation of all FILE(s) to standard output.

       Mandatory  arguments  to  long  options are mandatory for short options
       too.  Ordering options:

       -r, --reverse
              reverse the result of comparisons

       -c, --check
              check whether input is sorted; do not sort

       -m, --merge
              merge already sorted files; do not sort

       -o, --output=FILE
              write result to FILE instead of standard output

       -u, --unique
              with  -c, check for strict ordering; without -c, output only the
              first of an equal run

       --help display this help and exit

       --version
              output version information and exit

AUTHOR
       Written by Matt Gathu


"#;

#[derive(Debug)]
struct Args {
    reverse: bool,
    check: bool,
    merge: bool,
    unique: bool,
    help: bool,
    version: bool,
}

fn parse_args(args: &Vec<String>) -> Args {
    let pargs = Args {
        reverse: args.contains(&"--reverse".to_string()),
        check: args.contains(&"--check".to_string()),
        merge: args.contains(&"--merge".to_string()),
        unique: args.contains(&"--unique".to_string()),
        help: args.contains(&"--help".to_string()),
        version: args.contains(&"--version".to_string()),
    };

    pargs
}

fn merge<T: PartialOrd + Clone>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let mut left = left.clone();
    let mut right = right.clone();

    while !left.is_empty() && !right.is_empty() {
        match left.first().partial_cmp(&right.first()).unwrap() {
            Ordering::Less => {
                result.push(left[0].clone());
                left.remove(0);

            },
            Ordering::Greater => {
                result.push(right[0].clone());
                right.remove(0);
            },
            Ordering::Equal => {
                //let (first, rest) = left.split_first().unwrap();
                result.push(left[0].clone());
                left.remove(0);
            },
        };
    }

    if !left.is_empty() {
        for elem in left.iter() {
            result.push(elem.clone());
        }
    }

    if !right.is_empty() {
        result.extend(right.to_owned());
    }

    result
}

fn merge_sort<T: PartialOrd + Clone>(unsorted: &Vec<T>) -> Vec<T> {
    if unsorted.len() <= 1usize {
        return unsorted.clone();
    }
    
    let mid = unsorted.len() / 2usize;
    let (left, right) = unsorted.split_at(mid);

    let left = merge_sort(&left.to_vec());
    let right = merge_sort(&right.to_vec());

    merge(&left, &right)
}

fn read_file(fname: &String) -> Vec<String> {
    let mut contents = Vec::new();

    match File::open(&fname){
        Ok(input) => {    
            for line in BufReader::new(input).lines() {
            contents.push(line.unwrap());
            };
        },
        Err(e) => {
            println!("rsort: open failed: {}: {}", fname, e);
        },
    };

    // empty vec will be returned in case of error.
    contents

}

fn main() {

    let args: Vec<String> = env::args().collect();

    let pargs = parse_args(&args);

    match args.len() {
        1 => {
            println!("rsort: no arguments passed!!");
            return;
        },
        _ => {},
    };

    let fname = &args.last().unwrap();

    if pargs.help {
        println!("{}", HELP);
        process::exit(0);
    }

    if pargs.check || pargs.merge {
        println!("Not implemented");
        process::exit(0);
    }

    let unsorted = read_file(&fname);

    let mut result = merge_sort(&unsorted);

    if pargs.reverse {
        result.reverse();
    }
    
    for line in &result {
        println!("{}", line);
    }
}


#[test]
fn test_basic() {
    assert_eq!(merge_sort(&vec!["world", "hello"]), vec!["hello", "world"]);
}
