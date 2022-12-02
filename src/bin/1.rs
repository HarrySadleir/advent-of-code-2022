
fn main() {
    let contents = std::fs::read_to_string("./src/bin/1_input.txt").unwrap();

    let mut sums: Vec<u32> = contents
        .split("\n\n")
        .map(|block| 
            block.split("\n").flat_map(|value| value.parse::<u32>()).sum()
        ).collect();

    sums.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", sums[0]);
    let val: u32 = sums.iter().take(3).sum();
    println!("Part 2: {:?}", val);
}