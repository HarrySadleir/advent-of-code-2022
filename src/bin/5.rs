use array2d::Array2D;

fn main() {
    let contents = std::fs::read_to_string("./src/bin/5_input.txt").unwrap();

    let stack_cols: Vec<Vec<Option<char>>> = Array2D::from_rows(&contents.lines().take(8).map(|row| {
        let chars: Vec<char> = row.chars().collect();
        let mut result: Vec<Option<char>> = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == ' ' {
                result.push(None);
            } else {
                result.push(Some(chars[i+1]));
            }

            i+=4;
        }
        println!("{:?}", result);
        return result
    }).collect::<Vec<Vec<Option<char>>>>()).as_columns();

    let mut stacks: Vec<Vec<char>> = stack_cols.iter().map(|stack: &Vec<Option<char>>| stack.iter().flat_map(|c| *c).rev().collect::<Vec<char>>()).collect();

    // PART A
    // contents.lines().skip(10).for_each(|line| {
    //     let commands = line.split(" ").flat_map(|word| word.parse::<usize>()).collect::<Vec<usize>>(); 
    //     let mut i = 0; 
    //     while i < commands[0] {
    //         let removed_value_opt: Option<char>;
    //         {
    //             removed_value_opt = stacks[commands[1] - 1].pop();
    //         }
    //         if let Some(removed_value) = removed_value_opt {
    //             stacks[commands[2] - 1].push(removed_value);
    //         }
    //         i+=1;
    //     }
    // });

    // Part B
    contents.lines().skip(10).for_each(|line| {
        let commands = line.split(" ").flat_map(|word| word.parse::<usize>()).collect::<Vec<usize>>(); 
        let mut i = 0; 
        let mut removed_values: Vec<char> = vec![];
        while i < commands[0] {
            if let Some(val) = stacks[commands[1] - 1].pop() {
                removed_values.push(val);
            }
            i+=1;
        }
        
        removed_values.iter().rev().for_each(|val| stacks[commands[2] - 1].push(*val));
    });

    let result: String = stacks.iter().map(|stack: &Vec<char>| *stack.last().unwrap_or_else(|| &&' ')).collect();
    println!("Result: {}", result);
}