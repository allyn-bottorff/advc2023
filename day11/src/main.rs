use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn diff(&self, other: &Self) -> i32 {
        let x_diff = self.x as i32 - other.x as i32;
        let y_diff = self.y as i32 - other.y as i32;

        x_diff.abs() + y_diff.abs()
    }

    fn big_diff(&self, other: &Self, empty_xs: &Vec<usize>, empty_ys: &Vec<usize>) -> i64 {
        let mut additional_x = 0;
        let mut additional_y = 0;
        let mut first: usize;
        let mut second: usize;
        if self.x > other.x {
            first = other.x;
            second = self.x;
        } else {
            first = self.x;
            second = other.x;
        }
        for x in empty_xs {
            if (first..second).contains(x) {
                additional_x += 1_000_000 - 1;
            }
        }
        let x_diff = additional_x + second as i64 - first as i64;
        if self.y > other.y {
            first = other.y;
            second = self.y;
        } else {
            first = self.y;
            second = other.y;
        }
        for y in empty_ys {
            if (first..second).contains(y) {
                additional_y += 1_000_000 - 1;
            }
        }

        let y_diff = additional_y + second as i64 - first as i64;

        x_diff + y_diff
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let mut empty_ys: Vec<usize> = Vec::new();
    let mut empty_xs: Vec<usize> = Vec::new();

    for y in 0..grid.len() {
        if !grid[y].contains(&'#') {
            empty_ys.push(y);
        }
    }
    println!("{:?}", empty_ys);

    for x in 0..grid[0].len() {
        let mut found_galaxy = false;
        for y in 0..grid.len() {
            if grid[y][x] == '#' {
                found_galaxy = true;
            }
        }
        if !found_galaxy {
            empty_xs.push(x);
        }
    }

    println!("{:?}", empty_xs);

    let mut found: Vec<Point> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                found.push(Point { x, y });
            }
        }
    }

    let sum: i64 = found
        .into_iter()
        .combinations_with_replacement(2)
        .map(|x| x[0].big_diff(&x[1], &empty_xs, &empty_ys))
        .filter(|x| x != &0)
        .sum();

    println!("sum: {:?}", sum);
}
