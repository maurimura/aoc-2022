use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let binding = &fs::read("./day-1/input")?;
    let content = String::from_utf8_lossy(binding);

    let mut max_calories_and_position = (0, 0);

    let mut top_three = vec![0, 0, 0];

    let mut chars = content.chars();

    let mut elf_current_position = 0;
    let mut current_amount = String::from("");
    let mut current_calories = 0;
    let mut previous_linebreak = false;

    loop {
        let maybe_char = chars.next();
        match maybe_char {
            Some(char) => {
                if char.is_whitespace() {
                    if previous_linebreak {
                        top_three.push(current_calories);

                        top_three.sort_by(|a, b| b.cmp(a));
                        top_three = top_three[0..3].into();
                        if current_calories > max_calories_and_position.0 {
                            max_calories_and_position = (current_calories, elf_current_position);
                        }
                        current_calories = 0;
                        current_amount = String::from("");

                        elf_current_position = elf_current_position + 1;
                    } else {
                        println!("current calories: {}", current_calories);
                        println!("current amount: {}", current_amount);

                        current_calories += current_amount.parse::<u32>()?;
                        previous_linebreak = true;
                        current_amount = String::from("");
                    }
                } else {
                    current_amount.push(char);
                    previous_linebreak = false;
                }
            }
            None => break,
        }
    }

    println!(
        "ELF: Calories: {} Position: {}",
        max_calories_and_position.0, max_calories_and_position.1
    );

    println!("Top 3: {:?}", top_three.iter().sum::<u32>());
    Ok(())
}
