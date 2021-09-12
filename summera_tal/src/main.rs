use std::io;
use std::io::prelude::*;

fn main() {

    // Handle and Store input
    let input = io::stdin();
    let mut lines = input
        .lock()
        .lines()
        .map(|line| line.ok().unwrap());

    // Get sequence size, first line
    let sequence_size = lines
        .next().unwrap()
        .parse::<usize>().unwrap();

    // Calculate the n / 2.
    // Odd numbers are ceilinged
    let maximum_amounts = (sequence_size + 1) / 2;

    // Get the sequence of numbers, second line
    let mut sequence = lines
        .next().unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    
    // Stores the sum of the maximum numbers
    let mut summation : usize = 0;

    // Loop through the sequence n * n/2 times finding the maximum, adding it to the sum then set that maximum = 0
    for _ in 0..maximum_amounts {
        let mut max = 0;
        let mut index = 0;

        for elem in 0..sequence_size {
            if sequence[elem] > max {
                max = sequence[elem];
                index = elem;
            }
        }
        summation += max;
        sequence[index] = 0;
    }

    println!("{}", summation);
}