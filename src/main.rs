#![allow(unused)]

extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
// use std::io::Write;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: <$source> <$target>");
        return;
    }

    let file = File::open(args().nth(1).unwrap()).unwrap();

    let mut input = BufReader::new(file);

    let output = File::create(args().nth(2).unwrap()).unwrap();

    // let mut e = GzEncoder::new(output, Compression::best());
    let mut e = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    let bytes = copy(&mut input, &mut e).unwrap();

    let output = e.finish().unwrap();

    let elapsed_tile = start.elapsed();

    println!("[Source length] : {:?}", input.get_ref().metadata().unwrap().len());
    println!("[Target length] : {:?}", output.metadata().unwrap().len());
    println!("[BYTES] {}", bytes);
    println!("[Elapsed Time] {}ms", elapsed_tile.as_millis());

}
