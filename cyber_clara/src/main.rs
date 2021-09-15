use std::io::prelude::*;
use std::collections::HashSet;

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

    // Store unique names (first, last)
    let mut unique_names : HashSet<(&String, &String)> = HashSet::with_capacity(name_amount); 
    
    // Go through each name
    for name in 1..name_amount + 1 {
        // First and last names are offseted by that amount of names (name_amount)
        unique_names.insert((&lines[name], &lines[name + name_amount]));
    }

    // Print unique names (duplicates are removed)
    println!("{}", unique_names.len());
}