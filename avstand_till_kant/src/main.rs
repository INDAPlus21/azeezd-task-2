use std::io::prelude::*;
use std::cmp;

fn main() {

    // Handle input
    let input = std::io::stdin();
    let mut lines = input
        .lock()
        .lines()
        .map(|n| n.ok().unwrap());

    // Get dimensions of the desired rectangle, first and only row
    let dimensions = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // extract row and dimensions as variables
    let rows = dimensions[0] + 1;
    let cols = dimensions[1] + 1;
    
    // Loop through ROWS x COLS times
    for row in 1..dimensions[0] + 1 {
        for col in 1..dimensions[1] + 1 {

            /*
                This creates the "stairway effect" each row has an index and the values cannot exceed it
                The minimum function compares an increasing value (index) and and decreasing value (rows - index)
                creating this double-sided stairway effect on each row
                This value is then compared with regards to the columns to make the effect on the columns as well
            */
            let val = cmp::min(
                cmp::min(row, rows - row),
                cmp::min(col, cols - col)
            );
            
            // Greater than 9 outputs a '.'
            if val <= 9 {
                print!("{}", val);
            }
            else {
                print!(".");
            }
        }
        println!(); // newline
    }
}
