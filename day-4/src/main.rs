use std::fs;

struct Interpreter(String);

impl Interpreter {
    fn contains(&self) -> u32 {
        let pairs = self
            .0
            .split(",")
            .map(|pairs| {
                pairs
                    .split("-")
                    .flat_map(|value| value.parse::<u32>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let first_pair = pairs.get(0).unwrap();
        let second_pair = pairs.get(1).unwrap();
        let start1 = first_pair.get(0).unwrap();
        let end1 = first_pair.get(1).unwrap();
        let start2 = second_pair.get(0).unwrap();
        let end2 = second_pair.get(1).unwrap();

        if start2 >= start1 && end2 <= end1 {
            return 1;
        }

        if start1 >= start2 && end1 <= end2 {
            return 1;
        }
        0
    }

    fn overlaps(&self) -> u32 {
        let pairs = self
            .0
            .split(",")
            .map(|pairs| {
                pairs
                    .split("-")
                    .flat_map(|value| value.parse::<u32>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let first_pair = pairs.get(0).unwrap();
        let second_pair = pairs.get(1).unwrap();
        let start1 = first_pair.get(0).unwrap();
        let end1 = first_pair.get(1).unwrap();
        let start2 = second_pair.get(0).unwrap();
        let end2 = second_pair.get(1).unwrap();

        if end1 >= start2 && start2 >= start1 || end2 >= start1 && start1 >= start2 {
            return 1;
        }

        0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read_to_string("./day-4/input")?;
    let total = content
        .lines()
        .map(|line| Interpreter(line.to_string()).contains())
        .reduce(|acc, curr| acc + curr);

    let total2 = content
        .lines()
        .map(|line| Interpreter(line.to_string()).overlaps())
        .reduce(|acc, curr| acc + curr);

    println!("Part 1: {}", total.unwrap());
    println!("Part 2: {}", total2.unwrap());
    Ok(())
}
