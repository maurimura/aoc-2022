use std::fs;

fn populate_stacks(stacks: &[&str]) -> Vec<Vec<char>> {
    let stacks_seeds = stacks.iter().collect::<Vec<_>>();

    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    stacks_seeds.iter().for_each(|stack_seed| {
        let mut position = 0;

        let mut chars = stack_seed.chars();
        loop {
            match chars.next() {
                Some(char) => {
                    if !char.is_whitespace() && char != '[' && char != ']' {
                        let stack_position = position / 4;
                        match stacks.get_mut(stack_position) {
                            Some(stack) => stack.push(char),
                            None => (),
                        }
                    }

                    position += 1;
                }
                None => break,
            }
        }
    });

    stacks.iter_mut().for_each(|stack| stack.reverse());
    stacks
}
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read_to_string("./day-5/input")?;

    let content = &content.lines().collect::<Vec<_>>();
    let stacks = &content[0..8];
    let instructions = &content[10..];
    let mut stacks = populate_stacks(stacks);

    let instructions = instructions
        .iter()
        .map(|line| {
            let instruction = line.split(" ").collect::<Vec<_>>();
            let amount = instruction.get(1).unwrap().parse::<usize>().unwrap();
            let from = instruction.get(3).unwrap().parse::<usize>().unwrap();
            let to = instruction.get(5).unwrap().parse::<usize>().unwrap();
            (amount, from - 1, to - 1)
        })
        .collect::<Vec<_>>();

    instructions.iter().for_each(|instruction| {
        let (mut amount, from, to) = instruction;

        let temp_stack = &mut vec![];
        let original_from_stack = stacks[*from..=*from][0].clone();
        let original_amount = amount;
        let must_reverse = amount > 1;
        loop {
            if amount == 0 {
                break;
            }

            let from_stack = &mut stacks[*from..=*from][0];
            let popped = from_stack.pop();
            match popped {
                Some(popped) => {
                    temp_stack.push(popped);
                    amount -= 1;
                }
                None => (),
            };
        }

        let to_stack = &mut stacks[*to..=*to][0];
        if must_reverse {
            temp_stack.reverse();
        }
        let temp_clone = temp_stack.clone();
        let to_clone = to_stack.clone();

        to_stack.append(temp_stack);
        if must_reverse {
            println!(
                "Original: {:?} {:?}\n Temp: {:?}\n to: {:?} \n to_after: {:?}\n",
                original_from_stack, original_amount, temp_clone, to_clone, to_stack
            );
        }
    });

    println!("{:?}", stacks);
    let result = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect::<Vec<_>>();

    let result = result.iter().collect::<String>();
    println!("{:?}", result);

    Ok(())
}
