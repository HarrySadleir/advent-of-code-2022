fn do_ranges_contain_eachother(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
    (a1 >= b1 && a2 <= b2) || (a1 <= b1 && a2 >= b2) 
}

fn do_ranges_overlap(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
    !(a2 < b1 || b2 < a1)
}

fn main() {
    let contents = std::fs::read_to_string("./src/bin/4_input.txt").unwrap();

    let res: u32 = contents.lines().map(|line| {
        let nums: Vec<u32> = line.split(",").flat_map(|word: &str| 
            word.split("-").map(|val| val.parse::<u32>().unwrap())
        ).collect();
        do_ranges_contain_eachother(nums[0], nums[1], nums[2], nums[3]) as u32
    }).sum();

    let res2: u32 = contents.lines().map(|line| {
        let nums: Vec<u32> = line.split(",").flat_map(|word: &str| 
            word.split("-").map(|val| val.parse::<u32>().unwrap())
        ).collect();
        do_ranges_overlap(nums[0], nums[1], nums[2], nums[3]) as u32
    }).sum();

    println!("Part 1: {}", res);
    println!("Part 2: {}", res2);
}