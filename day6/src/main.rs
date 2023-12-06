fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let times: Vec<i32> = content
        .lines()
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let distances: Vec<i32> = content
        .lines()
        .last()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    //Race records Vec<(time, distance_record)>
    let race_records: Vec<(i32, i32)> = std::iter::zip(times, distances).collect();

    println!("race records: {:?}", race_records);

    let mut margins = Vec::new();

    for r in &race_records {
        let w_press = find_winning_press_times(r);
        margins.push(w_press.len());
    }

    let margin = margins.iter().copied().reduce(|acc, e| acc * e).unwrap();

    println!("margin: {}", margin);
}

fn find_winning_press_times(race: &(i32, i32)) -> Vec<i32> {
    /*
     * d = v * t
     * v = (press)
     * t = race_time - press
     * d = press * (race_time - press)
     *
     */

    let (r_time, r_record) = race;

    let mut winning_press_times = Vec::new();

    for p in 1..=*r_time {
        if p * (r_time - p) > *r_record {
            winning_press_times.push(p);
        }
    }

    winning_press_times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winning_press_times() {
        let winning_times = find_winning_press_times(&(7, 9));

        assert_eq!(vec![2, 3, 4, 5], winning_times)
    }
}
