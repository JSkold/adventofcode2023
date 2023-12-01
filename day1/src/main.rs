use std::{collections::BTreeMap, io::stdin};

fn main() {
    let matchers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let mut sum = 0;
    let mut sum2 = 0;

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        // PART 1
        let mut nums = line.matches(char::is_numeric);
        let first = nums.next().unwrap().parse::<usize>().unwrap();
        let last = nums
            .last()
            .map(|c| c.parse::<usize>().unwrap())
            .unwrap_or(first);

        // PART 2
        let mut matches: BTreeMap<usize, usize> = BTreeMap::new();
        for (v, m) in matchers.iter().enumerate() {
            for (index, _) in line.match_indices(m) {
                matches.insert(index, (v % 9) + 1);
            }
        }
        let first2 = matches.pop_first().unwrap().1;
        let last2 = matches.pop_last().unwrap_or((0, first2)).1;

        sum += first * 10 + last;
        sum2 += first2 * 10 + last2;
    }

    println!("{sum}");
    println!("{sum2}");
}
