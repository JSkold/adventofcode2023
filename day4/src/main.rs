use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut sum = 0;
    let mut sum2 = 0;
    let mut extra_cards = VecDeque::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut it = line.split(&[':', '|']).skip(1).map(|s| s.trim());

        // Collect winning numbers into Vec.
        let winning_nums = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // Collect ticket numbers into Vec.
        let ticket_nums = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // Get the number of wins for this ticket.
        let mut number_of_wins: usize = 0;
        for win_num in winning_nums {
            if ticket_nums.contains(&win_num) {
                number_of_wins += 1;
            }
        }

        let extra_card_factor = extra_cards.pop_front().unwrap_or(0);
        if number_of_wins > 0 {
            // Part 1 summation.
            sum += 2_usize.pow(number_of_wins as u32 - 1);

            // Part 2 summation.
            sum2 += number_of_wins * (extra_card_factor + 1);
            for i in 0..number_of_wins {
                match extra_cards.get_mut(i) {
                    Some(n) => *n += extra_card_factor + 1,
                    // Since we process the cards in order we know that whenever a entry in missing
                    // it will be at the back of the VecDeque.
                    None => extra_cards.push_back(extra_card_factor + 1),
                }
            }
        }
        // Add contribution from non-winning tickets as well.
        sum2 += 1;
    }

    println!("{sum}");
    println!("{sum2}");
}
