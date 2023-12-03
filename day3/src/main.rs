use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt");
}


/// Read all lines into a Vec<String> crash if something isn't right
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    buf.lines().map(|l| l.unwrap()).collect()

}

#[cfg(test)]
mod tests {
    // use super::*;
}
