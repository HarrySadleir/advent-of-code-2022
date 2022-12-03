use std::{collections::HashSet};

fn ch_to_value(ch: &char) -> u32 {
    if ch.is_ascii_uppercase() {
        *ch as u32 - 0x40 + 26
    } else {
        *ch as u32 - 0x60
    }
}

fn main() {
    let contents = std::fs::read_to_string("./src/bin/3_input.txt").unwrap();

    let res: u32 = contents.split("\n").map(|line: &str| {
        let len = line.len();
        let c1: &str = &line[..(len/2)];
        let c2: &str = &line[(len/2)..];
        let mut dupchars: HashSet<char> = HashSet::new();
        c1.chars().for_each(|ch| {
            if c2.contains(ch) {
                dupchars.insert(ch);
            }
        });
        let sm: u32 = dupchars.iter().map(|value| ch_to_value(value)).sum();
        sm
    }).sum();


    let vec2: Vec<&str> = contents.split("\n").collect();
    let mut i = 0;
    let mut sum: u32 = 0;
    while  i < vec2.len() {
        let mut dupchars: HashSet<char> = HashSet::new();
        vec2[i + 0].chars().for_each(|ch| {
            if vec2[i + 1].contains(ch) {
                dupchars.insert(ch);
            }
        });

        sum += dupchars
            .iter()
            .filter(|ch| vec2[i + 2].chars().collect::<Vec<char>>().contains(ch))
            .map(|ch| ch_to_value(&ch))
            .sum::<u32>();
            
        i+=3
    }

    println!("Part 1: {}", res);
    println!("Part 2: {}", sum);
}
