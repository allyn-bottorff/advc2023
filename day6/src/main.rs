fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let times: Vec<i64> = content
        .lines()
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = content
        .lines()
        .last()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    //Race records Vec<(time, distance_record)>
    let race_records: Vec<(i64, i64)> = std::iter::zip(times, distances).collect();

    println!("race records: {:?}", race_records);

    let mut margins = Vec::new();

    for r in &race_records {
        let w_press_count = count_winning_press_times(r);
        margins.push(w_press_count);
    }

    let margin = margins.iter().copied().reduce(|acc, e| acc * e).unwrap();

    println!("margin: {}", margin);

    let race: (i64, i64) = (61677571,430103613071150);

    let count = count_winning_press_times(&race);

    println!("count: {}", count);

    
}


fn count_winning_press_times(race: &(i64, i64)) -> u64 {
    /*
     * d = v * t
     * v = (press)
     * t = race_time - press
     * d = press * (race_time - press)
     *
     */

    let (r_time, r_record) = race;
    let mut count = 0;


    for p in 1..=*r_time {
        if p * (r_time - p) > *r_record {
            count += 1;
        }
    }

    count

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_winning_press_times() {
        let count = count_winning_press_times(&(71530,940200));
        assert_eq!(71503, count)


    }
}
