// The aim of this task is to write functions for reading and writing sequences of bits.
//
//
//
// http://rosettacode.org/wiki/Bitwise_IO
use std::fs::File;
use std::io::prelude::*;

struct Bitswriter {
    bcount: u8,
    acc: u8,
    v: Vec<u8>,
}

impl Bitswriter {
    fn new() -> Bitswriter {
        Bitswriter {
            bcount: 0,
            acc: 0,
            v: Vec::new(),
        }
    }

    fn flush(&mut self) {
        self.v.push(self.acc);
        self.acc = 0;
        self.bcount = 0;
    }

    fn writebit(&mut self, bit: u8) {
        if self.bcount == 8 {
            &self.flush();
        }

        if bit > 0 {
            self.acc |= 1 << 7 - self.bcount;
        }

        self.bcount += 1;
    }

    fn writebits(&mut self, bits: u8, n: i32) {
        let mut n = n;

        while n > 0 {
            let bit: u8 = bits & 1 << n - 1;
            &self.writebit(bit);
            n -= 1;
        }
    }
}

struct Bitsreader {
    bcount: u8,
    acc: u8,
    v: Vec<u8>,
}

impl Bitsreader {
    fn new(v: Vec<u8>) -> Self {
        Self {
            bcount: 0,
            acc: 0,
            v: v,
        }
    }

    fn readbit(&mut self) -> u8 {
        if self.bcount == 0 {
            let a = self.v.pop().unwrap();
            self.acc = a as u8;
            self.bcount = 8;
        }
        let rv = (self.acc & (1 << self.bcount-1)) >> self.bcount-1;
        self.bcount -= 1;
        rv
    }

    fn readbits(&mut self, n: i32) -> u8{
        let mut n = n;
        let mut v = 0;
        
        while n > 0 {
            v = (v << 1) | self.readbit();
            n -= 1;
        }
        v
    }
}

fn read_bytes(fname: &str) -> Vec<u8> {
    let mut v = Vec::new();
    let mut f = File::open(fname).unwrap();
    f.read_to_end(&mut v).unwrap();
    v.reverse();

    v
}

fn write_bytes(fname: &str, v: Vec<u8>) {
    let mut f = File::create(fname).unwrap();
    let b: &[u8] = &v;

    f.write(b).unwrap();
}

fn main() {

    let d = "qwertykeyboard";
    let vv: Vec<u8> = d.bytes().collect();
    let mut bwriter = Bitswriter::new();

    for b in vv {
        bwriter.writebits(b, 7);
    }

    let wbits = bwriter.v.clone();

    write_bytes("test.dat", wbits);

    let rbits = read_bytes("test.dat");
    let mut breader = Bitsreader::new(rbits);
    loop {
        print!("{}", breader.readbits(7) as char);
        if breader.v.is_empty() {
            println!();
            break;
        }
    }


}

//#[test]
