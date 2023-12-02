use std::path::Path;
use std::fs::File;
use std::io::{self,BufRead};

fn main() {

}


//Read all lines into a vec of strings. Crash if something isn't right.
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let buf = io::BufReader::new(file);

    buf.lines().map(|l| l.unwrap()).collect()
}



#[cfg(test)]
mod tests {
    // use super::*;
}
