use std::{collections::HashMap, fs};

enum Actions {
    CD(String),
    Store(String, Option<u64>),
}
struct Parser(String);

impl Parser {
    fn parse(self) -> Option<Actions> {
        println!("{}", self.0);
        if self.0.starts_with("$") {
            let splitted_input = self.0.split(" ").collect::<Vec<_>>();
            let command = splitted_input.get(1);

            let action: Option<Actions> = match command {
                Some(command) => {
                    if command.starts_with("cd") {
                        let arg = splitted_input.get(2).unwrap();
                        println!("arg {:?}", command);
                        return Some(Actions::CD(arg.to_string()));
                    }
                    None
                }
                None => None,
            };

            return action;
        } else {
            let splitted_input = self.0.split(" ").collect::<Vec<_>>();
            if self.0.starts_with("dir") {
                let name = splitted_input.get(1).unwrap();
                return Some(Actions::Store(name.to_string(), None));
            } else {
                println!("Splitted Input \n: {:?}", splitted_input);
                let file_name = splitted_input.get(1).unwrap();
                let file_size = splitted_input.get(0).unwrap().parse().unwrap();
                return Some(Actions::Store(file_name.to_string(), Some(file_size)));
            }
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    file_system: HashMap<String, Vec<(String, Option<u64>)>>,
    cd: Vec<String>,
}

impl State {
    fn new() -> Self {
        State {
            file_system: HashMap::new(),
            cd: vec![],
        }
    }

    fn interpreter(&mut self, action: Actions) {
        match action {
            Actions::CD(dir) => {
                if dir.eq("..") {
                    self.cd.pop();
                } else {
                    self.cd.push(dir);
                }
            }
            Actions::Store(name, size) => {
                let key = format!("{}", self.cd.join("/"));
                let empty_vec: Vec<(String, Option<u64>)> = vec![];
                let mut values = self.file_system.get(&key).unwrap_or(&empty_vec).clone();
                values.push((name, size));
                self.file_system.insert(key, values.to_vec());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut state = State::new();
    fs::read_to_string("./day-7/input")?
        .lines()
        .for_each(|input| match Parser(input.into()).parse() {
            Some(action) => state.interpreter(action),
            None => {}
        });

    println!("{:?}", state.file_system);
    // loop {
    //     match state_by_keys.next() {
    //         Some(key) => state.file_system,
    //         None => break,
    //     }
    // }

    Ok(())
}
