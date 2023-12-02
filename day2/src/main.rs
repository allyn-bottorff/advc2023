use std::path::Path;
use std::fs::File;
use std::io::{self,BufRead};


#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    number: u32,
    picks: Vec<Pick>,
}
impl Game {
    fn new(line: String) -> Self{
        let mut game = Game {
            number: 0,
            picks: Vec::new(),
        };
        let (game_str, picks) = line.split_once(":").unwrap();
        let (_, number) = game_str.split_once(" ").unwrap();
        game.number = number.parse::<u32>().unwrap();

        for pick_str in picks.split(";") {
            let mut pick = Pick{
                red: 0,
                green: 0,
                blue: 0,
            };
            for color in pick_str.split(",") {
                let color_parts: Vec<&str>= color.split(" ").collect();
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

    // let lines = read_lines("input.txt");
    let game1_text = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

    let game = Game::new(game1_text);



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
    #[test]
/*
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*/
    fn test_pt1_parse_game() {
        let game1_text = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        let game = Game::new(game1_text);





    }

}
