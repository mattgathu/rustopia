// http://rosettacode.org/wiki/HTTP
// task: Access and print a URL's content (the located resource) to the console.
#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;


fn main() {

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
