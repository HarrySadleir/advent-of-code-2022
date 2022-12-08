/*
    slice starts at a "$ cd foldername"

    returns the length until it's closing "cd .."
*/
fn get_cd_block_size(slice: &[&str]) -> usize {
    let mut i = 0;
    let mut stack_size = 0;
    for line in slice {
        if line.starts_with("$ cd ") {
            if line.contains("..") {
                stack_size -= 1;
            } else {
                stack_size += 1;
            }
        }

        if stack_size == 0 {
            break;
        }
        i += 1;
    }
    i
}

fn get_total_size_of_slice(slice: &[&str]) -> u32 {
    let mut res = 0;
    for line in slice {
        if let Ok(val) = line.split(" ").collect::<Vec<&str>>().first().unwrap().parse::<u32>() {
            res += val;
        }
    }

    res
}

fn main() {
    let contents = std::fs::read_to_string("./src/bin/7_input.txt").unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let mut i = 0;
    let mut all_sizes: Vec<u32> = Vec::new();
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("$ cd ") && line != "$ cd .." {
            let block_length = get_cd_block_size(&lines[i..]);
            all_sizes.push(get_total_size_of_slice(&lines[i..(i+block_length)]));
        } 

        i+=1;
    }

    let p1: u32 = all_sizes.iter().filter(|val| val <= &&100000).sum();

    let total_used: u32 = *all_sizes.iter().max().unwrap();
    let max_space: u32 = 40000000;
    let mut best_file_to_delete: u32 = 40000000;
    all_sizes.iter().for_each(|value| {
        if total_used - value < max_space && *value < best_file_to_delete {
            best_file_to_delete = *value;
        }
    });

    println!("Part 1: {}", p1);
    println!("Part 2: {}", best_file_to_delete);
}