use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> () {
    // Variables
    let mut sum_vec: Vec<i32> = vec![];
    let mut current_sum = 0;

    match read_lines("./src/data.txt") {
        Ok(lines) => {
            for line_result in lines {

                if let Ok(line) = line_result {
                    if line.trim().is_empty() {
                        sum_vec.push(current_sum);
                        current_sum = 0;
                        
                    } else {
                        current_sum += line.trim().parse::<i32>().unwrap();
                    }
                }
            }
        },

        Err(err) => {
            print!("Error opening file: {}", err);
        }
    }

    sum_vec.sort(); // Sort the vector

    // :? formtter can be used to debug
    println!("{:?}", sum_vec);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}