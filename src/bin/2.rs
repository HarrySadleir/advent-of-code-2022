enum RPS {
    ROCK,
    PAPER,
    SCISSORS
}

fn letter_to_rps(letter: &str) -> RPS {
    if letter == "A" || letter == "X" {
        RPS::ROCK
    } else if letter == "B" || letter == "Y" {
        RPS::PAPER
    } else {
        RPS::SCISSORS
    } 
}

fn get_score(yours: &RPS, theirs: &RPS) -> u32 {
    match yours {
        RPS::ROCK => match theirs {
            RPS::ROCK => 4,
            RPS::PAPER => 1,
            RPS::SCISSORS => 7
        },
        RPS::PAPER => match theirs {
            RPS::ROCK => 8,
            RPS::PAPER => 5,
            RPS::SCISSORS => 2
        },
        RPS::SCISSORS => match theirs {
            RPS::ROCK => 3,
            RPS::PAPER => 9,
            RPS::SCISSORS => 6
        }
    }
}

// This is hacky, but I already mapped it to an rps
fn throw_to_result(letter: &RPS) -> u32 {
    match letter {
        RPS::ROCK => 0,
        RPS::PAPER => 1,
        RPS::SCISSORS => 2
    }
}

// Result is 0 Loss, 1 draw, 2 win
fn derive_throw(theirs: &RPS, result: u32) -> RPS {
    match theirs {
        RPS::ROCK => match result {
            0 => RPS::SCISSORS,
            1 => RPS::ROCK,
            2 => RPS::PAPER,
            _ => RPS::PAPER
        },
        RPS::PAPER => match result {
            0 => RPS::ROCK,
            1 => RPS::PAPER,
            2 => RPS::SCISSORS,
            _ => RPS::PAPER
        },
        RPS::SCISSORS => match result {
            0 => RPS::PAPER,
            1 => RPS::SCISSORS,
            2 => RPS::ROCK,
            _ => RPS::PAPER
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("./src/bin/2_input.txt").unwrap();

    let result: u32 = contents
        .split("\n")
        .map(|row: &str| {
            let x: Vec<RPS> = row.split(" ").map(|character: &str| letter_to_rps(character)).collect();
            return get_score(&x[1], &x[0]);
        }).sum();

    let result2: u32 = contents
        .split("\n")
        .map(|row: &str| {
            let x: Vec<RPS> = row.split(" ").map(|character: &str| letter_to_rps(character)).collect();
            return get_score(&derive_throw(&x[0], throw_to_result(&x[1])), &x[0]);
        }).sum();

    println!("Part A: {:?}", result);
    println!("Part B: {:?}", result2);
}