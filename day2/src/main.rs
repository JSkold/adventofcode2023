use std::{cmp::max, io::stdin};

const COLOURS: &[&'static str] = &["red", "green", "blue"];

#[derive(Debug)]
struct Set {
    r: usize,
    g: usize,
    b: usize,
}

// Parses a game line, returns the game number and the set of cubes during each draw.
fn parse_game(mut line: String) -> (usize, Vec<Set>) {
    // Remove all space chars.
    line.retain(|c| c != ' ');

    // Parse the game number
    let game_number = line
        .split(':')
        .next()
        .unwrap()
        .strip_prefix("Game")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut sets = vec![];
    // Split on each draw, skip first since that is game number.
    let draws = line.split(&[':', ';']).skip(1);

    for draw in draws {
        let mut set = Set { r: 0, g: 0, b: 0 };
        let mut colour_split = draw.split(',');

        'colour_split: while let Some(colour_str) = colour_split.next() {
            for colour in COLOURS {
                if colour_str.ends_with(colour) {
                    // We matched a colour, strip it and parse the rest as the number of that colour.
                    let n = colour_str
                        .strip_suffix(colour)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    // Ugly colour str to struct member mapping.
                    match colour {
                        &"red" => set.r = n,
                        &"green" => set.g = n,
                        &"blue" => set.b = n,
                        _ => unreachable!(),
                    }
                    // If we match a colour we continue to the next one.
                    continue 'colour_split;
                }
            }
        }

        sets.push(set);
    }

    (game_number, sets)
}

fn main() {
    let mut sum = 0;
    let mut sum2 = 0;

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let (n, sets) = parse_game(line);

        // Part 1
        if sets.iter().all(|s| s.r <= 12 && s.b <= 14 && s.g <= 13) {
            sum += n;
        }

        // Part 2
        let mut minimum_set = Set { r: 0, g: 0, b: 0 };
        sets.iter().for_each(|s| {
            minimum_set.r = max(s.r, minimum_set.r);
            minimum_set.b = max(s.b, minimum_set.b);
            minimum_set.g = max(s.g, minimum_set.g);
        });
        sum2 += minimum_set.r * minimum_set.b * minimum_set.g;
    }

    println!("{sum}");
    println!("{sum2}");
}
