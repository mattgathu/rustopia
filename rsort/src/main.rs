use std::env;
use std::fs::File;
use std::cmp::Ordering;
use std::io::{BufReader, BufRead};

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
    let fname = &args[1];

    let unsorted = read_file(&fname);

    let result = merge_sort(&unsorted);
    
    for line in &result {
        println!("{}", line);
    }
}


#[test]
fn test_basic() {
    assert_eq!(merge_sort(&vec!["world", "hello"]), vec!["hello", "world"]);
}
