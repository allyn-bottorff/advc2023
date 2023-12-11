use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
///H: ─, V: │
///UL: ┌, UR: ┐
///DL: └, DR: ┘
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

fn connected_points(pipe: &Pipe, x_len: &usize, y_len: &usize) -> (Option<Point>, Option<Point>) {
    let (p1, p2): (Option<Point>, Option<Point>) = match pipe {
        Pipe::V(p) => {
            let mut p1: Option<Point> = None;
            let mut p2: Option<Point> = None;
            if p.y < *y_len - 1 {
                p1 = Some(Point { x: p.x, y: p.y + 1 });
            }
            if p.y > 0 {
                p2 = Some(Point { x: p.x, y: p.y - 1 });
            }
            (p1, p2)
        }
        Pipe::H(p) => {
            let mut p1: Option<Point> = None;
            let mut p2: Option<Point> = None;
            if p.x < *x_len - 1 {
                p1 = Some(Point { x: p.x + 1, y: p.y });
            }
            if p.x > 0 {
                p2 = Some(Point { x: p.x - 1, y: p.y });
            }
            (p1, p2)
        }
        Pipe::DR(p) => {
            let mut p1: Option<Point> = None;
            let mut p2: Option<Point> = None;
            if p.x > 0 {
                p1 = Some(Point { x: p.x - 1, y: p.y });
            }
            if p.y > 0 {
                p2 = Some(Point { x: p.x, y: p.y - 1 });
            }
            (p1, p2)
        }
        Pipe::DL(p) => {
            let mut p1: Option<Point> = None;
            let mut p2: Option<Point> = None;
            if p.x < *x_len - 1 {
                p1 = Some(Point { x: p.x + 1, y: p.y });
            }
            if p.y > 0 {
                p2 = Some(Point { x: p.x, y: p.y - 1 });
            }
            (p1, p2)
        }
        Pipe::UR(p) => {
            let mut p1: Option<Point> = None;
            let mut p2: Option<Point> = None;
            if p.x > 0 {
                p1 = Some(Point { x: p.x - 1, y: p.y });
            }
            if p.y < *y_len - 1 {
                p2 = Some(Point { x: p.x, y: p.y + 1 });
            }
            (p1, p2)
        }
        Pipe::UL(p) => {
            let mut p1: Option<Point> = None;
            let mut p2: Option<Point> = None;
            if p.x < *x_len - 1 {
                p1 = Some(Point { x: p.x + 1, y: p.y });
            }
            if p.y < *y_len - 1 {
                p2 = Some(Point { x: p.x, y: p.y + 1 });
            }
            (p1, p2)
        }
        Pipe::DOT(_) => (None, None),
        Pipe::S(_) => (None, None),
    };
    (p1, p2)
}

fn match_pipe(c: &char, p: &Point) -> Pipe {
    match c {
        '─' => Pipe::H(p.clone()),
        '│' => Pipe::V(p.clone()),
        '┌' => Pipe::UL(p.clone()),
        '┐' => Pipe::UR(p.clone()),
        '└' => Pipe::DL(p.clone()),
        '┘' => Pipe::DR(p.clone()),
        'S' => Pipe::S(p.clone()),
        '.' => Pipe::DOT(p.clone()),
        _ => panic!("found unexpected char"),
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
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let contents = contents.replace("-", "─");
    let contents = contents.replace("|", "│");
    let contents = contents.replace("F", "┌");
    let contents = contents.replace("7", "┐");
    let contents = contents.replace("L", "└");
    let contents = contents.replace("J", "┘");

    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    // println!("{contents}");
    print_grid(&grid);

    let y_len = grid.len();
    let x_len = grid[0].len();

    let mut start = Point { x: 0, y: 0 };
    let mut pipe_start = Point { x: 0, y: 0 };

    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == 'S' {
                println!("Found start at: {},{}", x, y);
                start = Point { x, y }
            }
        }
    }

    let mut connected = false;
    if start.x > 0 {
        let test = Point {
            x: start.x - 1,
            y: start.y,
        };
        let pipe = match_pipe(&grid[test.y][test.x], &test);
        let connected_points = connected_points(&pipe, &x_len, &y_len);
        if match connected_points.0 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }
        if match connected_points.1 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }

        if connected {
            pipe_start = test;
        }
    }
    if start.x < x_len - 1 && !connected {
        let test = Point {
            x: start.x + 1,
            y: start.y,
        };
        let pipe = match_pipe(&grid[test.y][test.x], &test);
        let connected_points = connected_points(&pipe, &x_len, &y_len);
        if match connected_points.0 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }
        if match connected_points.1 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }

        if connected {
            pipe_start = test;
        }
    }
    if start.y > 0 && !connected {
        let test = Point {
            x: start.x,
            y: start.y - 1,
        };
        let pipe = match_pipe(&grid[test.y][test.x], &test);
        let connected_points = connected_points(&pipe, &x_len, &y_len);
        if match connected_points.0 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }
        if match connected_points.1 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }
        if connected {
            pipe_start = test;
        }
    }
    if start.y < y_len - 1 && !connected {
        let test = Point {
            x: start.x,
            y: start.y + 1,
        };
        let pipe = match_pipe(&grid[test.y][test.x], &test);
        println!("{:?}", pipe);
        let connected_points = connected_points(&pipe, &x_len, &y_len);
        println!("{:?}", connected_points);
        if match connected_points.0 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }
        if match connected_points.1 {
            Some(p) => p == start,
            None => false,
        } {
            connected = true;
        }
        if connected {
            pipe_start = test;
        }
    }

    println!("pipe_start: {:?}", pipe_start);

    let mut from = start.clone();
    let mut current: Point = pipe_start.clone();
    let mut steps: i32 = 0;
    while current != start {
        let current_pipe = match_pipe(&grid[current.y][current.x], &current);
        let connected = connected_points(&current_pipe, &x_len, &y_len);

        match connected.0 {
            Some(ref p) => {
                if *p == from {
                    from = current;
                    current = connected.1.clone().unwrap();
                }
            }
            None => println!("found broken loop at: {},{}", current.y, current.x),
        }

        match connected.1 {
            Some(ref p) => {
                if *p == from {
                    from = current;
                    current = connected.0.clone().unwrap();
                }
            }
            None => println!("found broken loop at: {},{}", current.y, current.x),
        }
        steps += 1;
    }

    println!("total steps in the loop: {}", steps);
    println!("farthest distance: {}", steps / 2 + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected_points1() {
        let point = Point { x: 1, y: 1 };
        let pipe = match_pipe(&'─', &point);
        let connected = connected_points(&pipe, &5, &5);

        let expected_point_1 = Point { x: 0, y: 1 };
        let expected_point_2 = Point { x: 2, y: 1 };

        assert_eq!(connected.0.unwrap(), expected_point_2);
        assert_eq!(connected.1.unwrap(), expected_point_1);
    }
    #[test]
    fn test_connected_points2() {
        let point = Point { x: 1, y: 1 };
        let pipe = match_pipe(&'│', &point);
        let connected = connected_points(&pipe, &5, &5);

        let expected_point_1 = Point { x: 1, y: 0 };
        let expected_point_2 = Point { x: 1, y: 2 };

        assert_eq!(connected.0.unwrap(), expected_point_2);
        assert_eq!(connected.1.unwrap(), expected_point_1);
    }
    #[test]
    fn test_connected_points3() {
        let point = Point { x: 1, y: 1 };
        let pipe = match_pipe(&'┌', &point);
        let connected = connected_points(&pipe, &5, &5);

        let expected_point_1 = Point { x: 2, y: 1 };
        let expected_point_2 = Point { x: 1, y: 2 };

        assert_eq!(connected.0.unwrap(), expected_point_1);
        assert_eq!(connected.1.unwrap(), expected_point_2);
    }
}
