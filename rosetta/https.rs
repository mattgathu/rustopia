// https://rosettacode.org/wiki/HTTPS
// task: Access and print a URL's content (the located resource) to the console.
extern crate curl;
extern crate reqwest;

use std::io::{self, Read, Write};
use curl::easy::Easy;


// using curl-rust
fn curl() {
    let mut easy = Easy::new();
    easy.url("https://httpbin.org/ip").unwrap();
    easy.write_function(|data| {
        Ok(io::stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("Response: {}", easy.response_code().unwrap());
}

// using reqwest
fn reqwest(){
    let mut resp = reqwest::get("https:httpbin.org/ip").unwrap();
    println!("Response: {}", resp.status());

    let mut content = String::new();
    let _ = resp.read_to_string(&mut content);

    println!("{}", content);
}


fn main () {
    println!("Using: Curl-rust");
    curl();

    println!("Using: Reqwest");
    reqwest();
}
