// http://rosettacode.org/wiki/HTTP
// task: Access and print a URL's content (the located resource) to the console.
#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate curl;
extern crate reqwest;

use std::io::{self, Read, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use curl::easy::Easy;


// using curl-rust
fn curl() {
    let mut easy = Easy::new();
    easy.url("http://httpbin.org/ip").unwrap();
    easy.write_function(|data| {
        Ok(io::stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("Response: {}", easy.response_code().unwrap());
}

// using hyper
fn hyper() {

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = "http://httpbin.org/ip".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });

    core.run(work).unwrap();
}

// using reqwest
fn reqwest(){
    let mut resp = reqwest::get("http:httpbin.org/ip").unwrap();
    println!("Response: {}", resp.status());

    let mut content = String::new();
    let _ = resp.read_to_string(&mut content);

    println!("{}", content);
}


fn main () {
    println!("Using: Hyper");
    hyper();

    println!("Using: Curl-rust");
    curl();

    println!("Using: Reqwest");
    reqwest();
}
