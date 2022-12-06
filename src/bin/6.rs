use std::collections::{ VecDeque};
fn main() {
    let contents = std::fs::read_to_string("./src/bin/6_input.txt").unwrap();
    let chars: Vec<char> = contents.chars().collect();

    let mut i: usize = 3;
    let mut buffer: VecDeque<char> = chars.iter().take(4).rev().map(|cha: &char| *cha).collect();
    let mut next_cha: char;
    while i < chars.len() {
        next_cha = chars[i];
        if !buffer.iter().fold(false, |rsf, cha: &char| rsf || buffer.iter().fold(false, |rsf, cha2: &char| {
            rsf || if std::ptr::eq(cha, cha2) {
                false
            } else {
                *cha == *cha2
            } 
        })) {
            break;
        }
        buffer.pop_back();
        buffer.push_front(next_cha);
        i+=1;
    };
    println!("Part 1: {:?}", i);

    i = 13;
    buffer = chars.iter().take(14).rev().map(|cha: &char| *cha).collect();
    let mut next_cha: char;
    while i < chars.len() {
        next_cha = chars[i];
        if !buffer.iter().fold(false, |rsf, cha: &char| rsf || buffer.iter().fold(false, |rsf, cha2: &char| {
            rsf || if std::ptr::eq(cha, cha2) {
                false
            } else {
                *cha == *cha2
            } 
        })) {
            break;
        }
        buffer.pop_back();
        buffer.push_front(next_cha);
        i+=1;
    };
    println!("Part 2: {:?}", i);
}