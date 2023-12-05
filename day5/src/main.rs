use std::{
    cmp::min,
    io::{stdin, Read},
    ops::Range,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let seeds: Vec<usize> = input
        .split(&[':', '\n'])
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // Store each map as a Vec of ranges + dest. Each range represent the source mappings.
    // The dest is the beginning of the destination mapping.
    let mut range_vecs: Vec<Vec<(Range<usize>, usize)>> = Vec::new();

    let mut input_it = input.split("map:").skip(1);
    while let Some(it_str) = input_it.next() {
        range_vecs.push(Vec::new());

        let mut map_line_split = it_str.split("\n\n").next().unwrap().trim().split('\n');
        while let Some(map_line) = map_line_split.next() {
            let mut map_split = map_line.split_ascii_whitespace();
            let dest_idx: usize = map_split.next().unwrap().parse().unwrap();
            let src_idx: usize = map_split.next().unwrap().parse().unwrap();
            let range: usize = map_split.next().unwrap().parse().unwrap();

            range_vecs
                .last_mut()
                .unwrap()
                .push((src_idx..src_idx + range, dest_idx));
        }
    }

    // Part 1
    let mut lowest = None;
    for seed in seeds.iter() {
        let mut next = *seed;
        'next_map: for ranges in range_vecs.iter() {
            for range in ranges {
                next = match range.0.contains(&next) {
                    true => {
                        next = next + range.1 - range.0.start;
                        continue 'next_map;
                    }
                    false => next,
                }
            }
        }

        lowest = match lowest {
            Some(now) => Some(min(now, next)),
            None => Some(next),
        };
    }
    println!("{}", lowest.unwrap());

    // Part 2
    let seed_ranges: Vec<Range<usize>> = seeds
        .chunks_exact(2)
        .map(|r| *r.get(0).unwrap()..(*r.get(0).unwrap() + *r.get(1).unwrap()))
        .collect();

    let mut i = 0;
    loop {
        let mut next = i;
        // We solve this problem by going backwards through the maps and attempt to
        // find the smallest location value that gives us a seed in any of input ranges.
        'next_map: for ranges in range_vecs.iter().rev() {
            for range in ranges {
                if (range.1..(range.1 + range.0.end - range.0.start)).contains(&next) {
                    next = next - range.1 + range.0.start;
                    continue 'next_map;
                }
            }
        }

        // Check if the seed we got from the location is in any of the valid ranges.
        if seed_ranges.iter().any(|r| r.contains(&next)) {
            println!("{i}");
            break;
        }
        i += 1;
    }
}
