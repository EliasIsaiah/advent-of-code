// use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

// fn print_lines() {
// }

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;
    for num in vec {
        println!("{}", num);
    }
    Ok(())
}

// for depth in &depths_ints {
//     println!("{}", depth);
// }

// let numbers: Vec<i32> = input;
//     .split_whitespace()
//     .map(|s| s.parse().expect("parse error"))
//     .collect();

// for num in numbers {
//     println!("{}", num);
// }
