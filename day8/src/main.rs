use std::{
    collections::HashMap,
    io::{stdin, Read},
};

enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct Node {
    value: String,
    connections: (String, String),
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    let n = lcm(&nums[1..]);
    nums[0] * n / gcd(nums[0], n)
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut input_section = input.split("\n\n");

    // Parse directions line
    let dirs = input_section
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!(),
        })
        .collect::<Vec<Direction>>();

    // Parse node map
    let mut map: HashMap<String, Node> = HashMap::new();
    for line in input_section.next().unwrap().split('\n') {
        if line.len() == 0 {
            break;
        }
        let mut line_section = line.split('=');

        let node_value = line_section.next().unwrap().trim().to_string();
        let mut connections_split = line_section.next().unwrap().split(", ");
        let left = connections_split
            .next()
            .unwrap()
            .strip_prefix(" (")
            .unwrap()
            .to_string();
        let right = connections_split
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .to_string();
        let node = Node {
            value: node_value.clone(),
            connections: (left, right),
        };
        map.insert(node_value, node);
    }

    // Part 1, search from AAA to ZZZ
    let mut it = dirs.iter().cycle().enumerate();
    let mut current_node = map.get("AAA").unwrap();
    let n = loop {
        let (n, dir) = it.next().unwrap();

        if current_node.value == "ZZZ" {
            break n;
        }

        let next_node = match dir {
            Direction::Right => &current_node.connections.1,
            Direction::Left => &current_node.connections.0,
        };

        current_node = map.get(next_node).unwrap();
    };

    println!("{n}");

    // Part 2, search from **A to **Z. Collect individual lengths the find LCM.
    let mut steps = vec![];

    for n in map.keys().filter(|name| name.ends_with('A')) {
        let mut it = dirs.iter().cycle().enumerate();
        let mut current_node = map.get(n).unwrap();

        let n = loop {
            let (n, dir) = it.next().unwrap();

            if current_node.value.ends_with('Z') {
                break n;
            }

            let next_node = match dir {
                Direction::Right => &current_node.connections.1,
                Direction::Left => &current_node.connections.0,
            };

            current_node = map.get(next_node).unwrap();
        };

        steps.push(n);
    }

    println!("{}", lcm(&steps[..]));
}
