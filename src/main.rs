use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::fs::File;
use std::io::{BufReader};
use std::time::Instant;
use std::io::copy;

fn main() {
    if env::args().len() < 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let input: String = env::args().nth(1).unwrap();
    let mut input = BufReader::new(File::open(input).unwrap());
    let mut output: String = env::args().nth(2).unwrap();
    if !output.ends_with(".gz") {
        output += ".gz";
    }
    let mut output = File::create(output).unwrap();
    let mut encoder = GzEncoder::new(&mut output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Time elapsed: {:?}", start.elapsed());

}
