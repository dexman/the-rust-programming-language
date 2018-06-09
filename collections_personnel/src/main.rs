use std::io;
use std::collections::HashMap;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        let tokens = command.split_whitespace().collect::<Vec<&str>>();

        if tokens.is_empty() {
            // Exit when empty line entered.
            break;
        }

        let command = tokens[0];
        match command {
            "Get" => {
                let department_name = tokens[1];
                let department = employees.get(department_name);
                match department {
                    Some(people_names) => {
                        println!("{:?}", people_names);
                    }
                    None => {
                        println!("No department `{}` exists.", department_name);
                    }
                }
            }
            "Add" => {
                let person_name = String::from(tokens[1]);
                let department_name = String::from(tokens[3]);
                let people = employees.entry(department_name).or_insert(Vec::new());
                people.push(person_name);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
