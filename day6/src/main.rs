use std::io::{stdin, Error};

fn read_line(line: Option<Result<String, Error>>) -> Vec<usize> {
    line.unwrap()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect()
}

fn calc_dist_in_time(time_held: usize, max_time: usize) -> usize {
    let time_left = max_time - time_held;

    return time_held * time_left;
}

fn main() {
    let mut lines = stdin().lines();

    // Parsing
    let times = read_line(lines.next());
    let dist = read_line(lines.next());

    // Part 1
    let mut prod = Vec::new();
    for (race, time) in times.iter().enumerate() {
        let mut number_of_way_to_win = 0;
        for t in 0..=*time {
            if calc_dist_in_time(t, *time) > *dist.get(race).unwrap() {
                number_of_way_to_win += 1;
            }
        }
        prod.push(number_of_way_to_win);
    }
    println!("{}", prod.iter().product::<usize>());

    // Part 2
    // Hacky way to quickly get the combined number
    let new_time = times
        .into_iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let new_dist = dist
        .into_iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    // Just do it again for the larger numbers
    let mut number_of_way_to_win = 0;
    for t in 0..=new_time {
        if calc_dist_in_time(t, new_time) > new_dist {
            number_of_way_to_win += 1;
        }
    }
    println!("{number_of_way_to_win}");
}
