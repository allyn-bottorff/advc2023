fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let mut numbers: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let line_strs: Vec<&str> = line.split_whitespace().collect();

        let line_numbers: Vec<i32> = line_strs
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        numbers.push(line_numbers);
    }

    let mut next_numbers: Vec<i32> = Vec::new();

    for line in numbers {
        let mut found_zeroes = false;
        let mut history_vecs: Vec<Vec<i32>> = Vec::new();
        let mut line = line.clone();
        line.reverse();

        history_vecs.push(line);
        while !found_zeroes {
            let mut new_line: Vec<i32> = Vec::new();
            let current_line = history_vecs.last().unwrap();
            for i in 1..history_vecs.last().unwrap().len() {
                new_line.push(current_line[i] - current_line[i - 1]);
            }
            history_vecs.push(new_line);
            let mut non_zero = false;
            for item in history_vecs.last().unwrap() {
                if item != &0 {
                    non_zero = true;
                }
            }
            if non_zero == false {
                found_zeroes = true;
                let mut sum: i32 = 0;
                for i in 0..history_vecs.len() {
                    sum += history_vecs[i].last().unwrap()
                }
                next_numbers.push(sum);
            }
        }

        println!("history vecs: {:?}", history_vecs);
    }

    println!("");
    println!("next numbers: {:?}", next_numbers);
    println!("sum: {}", next_numbers.into_iter().sum::<i32>());
}
