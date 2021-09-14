use std::io::prelude::*;
use std::collections::HashMap;

// Store first + name
#[derive(Hash, Eq, PartialEq)]
struct FullName<'a>{
    first_name : &'a String,
    last_name : &'a String
}

fn main() {

    // Handle input
    let input = std::io::stdin();
    let lines = input
        .lock()
        .lines()
        .map(|line| line.ok().unwrap())
        .collect::<Vec<String>>();

    // Get the amount of names, first line
    let name_amount : usize = lines[0]
        .parse()
        .unwrap();

    // Store unique names (Unique First + Lastname)
    let mut unique_names : HashMap<FullName, usize> = HashMap::new();
    
    // Go through each name
    for name in 1..name_amount + 1 {

        // Create a full name struct (Offset between first name and last name in the input is constant (the number in the first line))
        let current_name = FullName {
            first_name : &lines[name],
            last_name: &lines[name + name_amount]
        };

        // Add name to HashMap if they are not already in there
        if !unique_names.contains_key(&current_name) {
            unique_names.insert(current_name, 1);
        }
    }

    // Print unique names (duplicates are removed)
    println!("{}", unique_names.len());
}