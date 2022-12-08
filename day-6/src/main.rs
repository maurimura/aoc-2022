use std::collections::HashSet;
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read_to_string("./day-6/input")?;

    let mut chars = content.chars();
    let mut position = 0;
    let mut queue: Vec<char> = vec![];
    // let limit = 4; // Part 1
    let limit = 14; // Part 2

    loop {
        match chars.next() {
            Some(char) => {
                position += 1;
                if queue.len() == limit - 1 {
                    let mut hash_set = queue.iter().collect::<HashSet<_>>();
                    hash_set.insert(&char);
                    if hash_set.len() == limit {
                        break;
                    } else {
                        queue.push(char);
                        queue = queue[1..limit].into();
                    }
                } else {
                    queue.push(char);
                }
            }
            None => break,
        }
    }
    println!("{position}");
    Ok(())
}
