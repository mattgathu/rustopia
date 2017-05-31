extern crate clap;

use std::env;
use clap::{Arg, App};
use std::os::unix::fs::MetadataExt;


fn main() {
    let matches = App::new("rls")
        .version("0.0.1")
        .author("Matt Gathu <mattgathu@gmail.com>")
        .about("simple ls command implemented in rust")
        .arg(Arg::with_name("1")
             .short("1")
             .help("Force output to be one entry per line."))
        .arg(Arg::with_name("i")
             .short("i")
             .help("For each file, print the file's file serial number (inode number)."))
        .arg(Arg::with_name("r")
             .short("r")
             .help("Reverse the order of the sort"))
        .arg(Arg::with_name("s")
             .short("s")
             .help("Display the number of file system blocks actually used by each
file, in units of 512 bytes."))
        .get_matches();

    let inode = matches.is_present("i");
    let size = matches.is_present("s");
    let reverse = matches.is_present("r");
    let one = matches.is_present("1");

    let path = env::current_dir().unwrap();
    let mut entries: Vec<_> = path.read_dir().unwrap().collect();

    if size {
        let total: u64 = path.read_dir()
            .unwrap()
            .map(|x| {
                x.unwrap()
                    .path()
                    .metadata()
                    .unwrap()
                    .blocks()
            })
            .sum();

        println!("total {}", total);
    }


    if reverse {
        entries.reverse();
    }

    fn print_out(s: String, one: bool) {
        if one {
            println!("{}", s);
        } else {
            print!("{}\t", s)
        }
    }

    for entry in entries {
        if let Ok(entry) = entry {
            let meta = entry.path().metadata().unwrap();
            if size && inode {
                print!("{} {} ", meta.ino(), meta.blocks());
            } else if size {
                print!("{} ", meta.blocks());
            } else if inode {
                print!("{} ", meta.ino());
            }
            print_out(format!("{}",
                              entry.path()
                                  .file_name()
                                  .unwrap()
                                  .to_str()
                                  .unwrap()),
                      one);
        }
    }
    println!();
}
