

struct Point {
    x: usize,
    y: usize,
}
enum Pipe {
    V(Point),
    H(Point),
    UR(Point),
    UL(Point),
    DR(Point),
    DL(Point),
    DOT(Point),
    S(Point),
}

fn is_connected(pipe: &Pipe, point: &Point) -> bool {
    match pipe {
        Pipe::V(p) => p.x == point.x && (p.y == point.y + 1 || p.y == point.y - 1 ),
        Pipe::H(p) => (p.x == point.x + 1 || p.x == point.x - 1) && p.y == point.y,
        Pipe::UR(p) = > (p.x == point.x + 

    }
}

type Grid = Vec<Vec<char>>;
fn print_grid(grid: &Grid) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    // ─ V
    // │ H
    // ┌ UR
    // ┐ UL
    // └ DR
    // ┘ DL

    let mut contents = std::fs::read_to_string("example.txt").unwrap();
    let contents = contents.replace("-", "─");
    let contents = contents.replace("|", "│");
    let contents = contents.replace("F", "┌");
    let contents = contents.replace("7", "┐");
    let contents = contents.replace("L", "└");
    let contents = contents.replace("J", "┘");

    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    println!("{contents}");
    print_grid(&grid);

}




