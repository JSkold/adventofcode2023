use std::{collections::HashMap, io::stdin};

/// Parses the number from a line given any index part of that number occupies.
/// Returns the starting index of the number and the value of the number as a tuple.
fn get_number_from_index(line: &str, mut idx: usize) -> (usize, usize) {
    let line_len = line.len();
    let mut num_len = 1;

    // Search to the right, break when c is not a digit anymore.
    let mut it = line.chars().skip(idx + 1);
    while let Some(c) = it.next() {
        if c.is_ascii_digit() {
            num_len += 1;
        } else {
            break;
        }
    }

    // Search to the left, break when c is not a digit anymore.
    let mut it = line.chars().rev().skip(line_len - idx);
    while let Some(c) = it.next() {
        if c.is_ascii_digit() {
            idx -= 1;
            num_len += 1;
        } else {
            break;
        }
    }

    let val = line[idx..idx + num_len].parse::<usize>().unwrap();
    (idx, val)
}

fn main() {
    // Collect into a Vec of strings since we need to reference the previous and next lines for every line.
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let line_len = lines.first().unwrap().len();

    let mut collected_numbers: HashMap<usize, usize> = HashMap::new();
    let mut sum2 = 0;

    for (line_num, line) in lines.iter().enumerate() {
        // Iterate over all ascii symbols except '.'.
        let mut sym_it = line.match_indices(|c: char| c != '.' && c.is_ascii_punctuation());

        while let Some((sym_idx, sym)) = sym_it.next() {
            let mut gear_num_vec = vec![];

            // Iterate over all adjacent positions.
            for y in line_num - 1..=line_num + 1 {
                for x in sym_idx - 1..=sym_idx + 1 {
                    // Get the char at the adjacent position.
                    if let Some(adj_char) =
                        lines.get(y).and_then(|line| line.chars().skip(x).next())
                    {
                        if adj_char.is_ascii_digit() {
                            let (idx, val) = get_number_from_index(lines.get(y).unwrap(), x);
                            // We use len*y+x as a hash to make sure we don't use the same number multiple times.
                            match collected_numbers.get(&(line_len * y + idx)) {
                                Some(_) => continue,
                                None => {
                                    collected_numbers.insert(line_len * y + idx, val);

                                    // Special handling of '*' for part 2. Uniqueness check of collected_numbers
                                    // ensures this isn't double counted.
                                    if sym == "*" {
                                        gear_num_vec.push(val);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if gear_num_vec.len() == 2 {
                sum2 += gear_num_vec.into_iter().product::<usize>();
            }
        }
    }

    let sum: usize = collected_numbers.into_values().sum();
    println!("{sum}");
    println!("{sum2}");
}
