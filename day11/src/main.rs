


#[derive(PartialEq)]
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
}


fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();

    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let mut galaxy_temp: Vec<Vec<char>> = Vec::new();

    let x_len = grid[0].len();

    for line in &grid {
        if !line.contains(&'#') {
            galaxy_temp.push(vec!['.'; x_len]);
        }
        galaxy_temp.push(line.clone());
    }

    let x_len = galaxy_temp[0].len();
    let y_len = galaxy_temp.len();
    let mut galaxy: Vec<Vec<char>> = Vec::new();
    for _i in 0..y_len {
        galaxy.push(Vec::new());
    }

    for x in 0..x_len {
        let mut found_galaxy = false;
        for y in 0..y_len {
            if galaxy_temp[y][x] == '#' {
                found_galaxy = true;
            }
        }
        for y in 0..y_len {
            if found_galaxy == false {
                galaxy[y].push('.');
            }
            galaxy[y].push(galaxy_temp[y][x]);
        }
    }

    for line in &grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    for line in &galaxy {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    let y_len = galaxy.len();
    let x_len = galaxy[0].len();

    let mut found: Vec<Point> = Vec::new();

    for y in 0..y_len {
        for x in 0..x_len {
            if galaxy[y][x] == '#' {
                found.push(Point { x, y });
            }
        }
    }

    let mut distances: Vec<i32> = Vec::new();

    for p in &found {
        for q in &found {
            if p != q {
                distances.push(p.diff(q));
            }
        }
    }

    println!("distances: {:?}", distances);
    println!("length of distances: {:?}", distances.len());
}
