use std::{
    collections::{BTreeSet, HashMap},
    io::stdin,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandOrd {
    High,
    OnePair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

impl HandOrd {
    fn get_from_vec(hand: &Vec<usize>) -> Self {
        let mut num_kind_map = HashMap::new();

        for c in hand {
            match num_kind_map.get_mut(c) {
                Some(n) => *n += 1,
                None => {
                    num_kind_map.insert(c, 1);
                }
            }
        }

        if num_kind_map.keys().any(|k| **k == 1) {
            // Ordering for part 2
            match num_kind_map.values().max() {
                Some(n) => match *n {
                    5 => Self::Five, // All card same
                    4 => Self::Five, // 4K + 1J => 5K, 4J + 1K => 5K
                    3 => match num_kind_map.get(&1).unwrap() {
                        1 => Self::Four, // 3K + 1J => 4K
                        2 => Self::Five, // 3K + 2J => 5K
                        3 => {
                            if num_kind_map.values().any(|n| *n == 2) {
                                Self::Five // 3J + 2K => 5K
                            } else {
                                Self::Four // 3J + 1K => 4K
                            }
                        }
                        _ => unreachable!(),
                    },
                    2 => match num_kind_map.get(&1).unwrap() {
                        1 => {
                            if num_kind_map.values().filter(|n| **n == 2).count() == 2 {
                                Self::House // 2K + 1J + 2K => 3K + 2K
                            } else {
                                Self::Three // 2K + 1J => 3K
                            }
                        }
                        2 => {
                            if num_kind_map.values().filter(|n| **n == 2).count() == 2 {
                                Self::Four // 2J + 2K => 2K + 2K
                            } else {
                                Self::Three // 2J + 1K => 3K
                            }
                        }
                        _ => unreachable!(),
                    },
                    1 => Self::OnePair, // All card unique, however one joker is present
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        } else {
            // Ordering for part 1
            match num_kind_map.values().max() {
                Some(n) => match *n {
                    5 => Self::Five,
                    4 => Self::Four,
                    3 => {
                        if num_kind_map.values().any(|n2| *n2 == 2) {
                            Self::House
                        } else {
                            Self::Three
                        }
                    }
                    2 => {
                        if num_kind_map.values().filter(|n2| **n2 == 2).count() == 2 {
                            Self::TwoPair
                        } else {
                            Self::OnePair
                        }
                    }
                    1 => Self::High,
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    v: Vec<usize>,
    bid: usize,
    order: HandOrd,
}

impl Hand {
    fn parse_from_string(str: &str, is_part_2: bool) -> Self {
        let mut str_split = str.split_ascii_whitespace();

        let v: Vec<usize> = str_split
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' if is_part_2 => 1,
                'J' => 11,
                'T' => 10,
                c if c.is_numeric() => c.to_digit(10).unwrap() as usize,
                _ => unreachable!(),
            })
            .collect();

        let bid = str_split.next().unwrap().parse().unwrap();

        let order = HandOrd::get_from_vec(&v);

        Self { v, bid, order }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            std::cmp::Ordering::Equal => self.v.iter().cmp(other.v.iter()),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut set = BTreeSet::new();
    let mut set2 = BTreeSet::new();
    let mut sum = 0;
    let mut sum2 = 0;

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let hand = Hand::parse_from_string(&line, false);
        set.insert(hand);

        let hand = Hand::parse_from_string(&line, true);
        set2.insert(hand);
    }

    set.iter()
        .enumerate()
        .for_each(|(i, h)| sum += h.bid * (i + 1));
    set2.iter()
        .enumerate()
        .for_each(|(i, h)| sum2 += h.bid * (i + 1));

    println!("{sum}");
    println!("{sum2}");
}
