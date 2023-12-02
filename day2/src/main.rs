use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}
impl Bag {
    fn from_game(game: &Game) -> Self {
        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;

        for pick in &game.picks {
            if pick.red > red_max {
                red_max = pick.red
            }
            if pick.blue > blue_max {
                blue_max = pick.blue
            }
            if pick.green > green_max {
                green_max = pick.green
            }
        }

        Bag {
            red: red_max,
            green: green_max,
            blue: blue_max,
        }
    }
    fn power(self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    picks: Vec<Pick>,
}
impl Game {
    fn new(line: &String) -> Self {
        let mut game = Game {
            number: 0,
            picks: Vec::new(),
        };
        let (game_str, picks) = line.split_once(":").unwrap();
        let (_, number) = game_str.split_once(" ").unwrap();
        game.number = number.parse::<u32>().unwrap();

        for pick_str in picks.split(";") {
            let mut pick = Pick {
                red: 0,
                green: 0,
                blue: 0,
            };
            for color in pick_str.split(",") {
                let color_parts: Vec<&str> = color.split(" ").collect();
                match color_parts[2] {
                    "blue" => pick.blue = color_parts[1].parse::<u32>().unwrap(),
                    "red" => pick.red = color_parts[1].parse::<u32>().unwrap(),
                    "green" => pick.green = color_parts[1].parse::<u32>().unwrap(),
                    _ => continue,
                }
            }
            game.picks.push(pick);
        }
        game
    }
}

#[derive(Debug)]
struct Pick {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let lines = read_lines("input.txt");
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum = 0;
    for line in &lines {
        let game = Game::new(line);
        if pt1_validate_game(&game, &bag) {
            sum += &game.number;
        }
    }

    println!("Part 1 sum: {}", sum);

    let mut sum = 0;
    for line in &lines {
        let game = Game::new(line);
        let bag = Bag::from_game(&game);
        sum += bag.power();
    }

    println!("Part 2 sum: {}", sum);
}

// Make sure all games are valid for the given bag contents
fn pt1_validate_game(game: &Game, bag: &Bag) -> bool {
    for p in &game.picks {
        if p.red > bag.red {
            return false;
        }
        if p.blue > bag.blue {
            return false;
        }
        if p.green > bag.green {
            return false;
        }
    }
    true
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
    use super::*;
    /*
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    */

    #[test]
    fn test_pt1_game_1_validate() {
        let game_text = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        let game = Game::new(&game_text);
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(pt1_validate_game(&game, &bag), true)
    }
    #[test]
    fn test_pt1_game_2_validate() {
        let game_text =
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");

        let game = Game::new(&game_text);
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(pt1_validate_game(&game, &bag), true)
    }
    #[test]
    fn test_pt1_game_3_validate() {
        let game_text = String::from(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );

        let game = Game::new(&game_text);
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(pt1_validate_game(&game, &bag), false)
    }
    #[test]
    fn test_pt1_game_4_validate() {
        let game_text = String::from(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );

        let game = Game::new(&game_text);
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(pt1_validate_game(&game, &bag), false)
    }
    #[test]
    fn test_pt1_game_5_validate() {
        let game_text = String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        let game = Game::new(&game_text);
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(pt1_validate_game(&game, &bag), true)
    }

    #[test]
    fn test_p2_bag_1() {
        let game_text = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game::new(&game_text);

        let bag = Bag::from_game(&game);

        let bag_literal = Bag {
            red: 4,
            green: 2,
            blue: 6,
        };
        assert_eq!(bag, bag_literal)
    }
}
