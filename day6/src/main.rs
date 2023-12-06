fn main() {
    let content = std::fs::read_to_string("example.txt").unwrap();
}

#[cfg(test)]
mod tests {}
