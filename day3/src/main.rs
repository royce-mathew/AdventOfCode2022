use std::fs;

fn get_priority(a: char) -> u32 {
    let byte = if a.is_lowercase() { 
        a as u32 - 'a' as u32 // Sets a as 0 b as 1
    } else {
         a as u32 - 'A' as u32 + 26 // sets A as 0+26
    };

    return byte + 1;
}

struct Solution;

impl Solution {
    fn sum(x: &str) -> u32 {
        let mut sum: u32 = 0;

        match fs::read_to_string(x) {
            Ok(line_as_string) => {
                for line in line_as_string.split("\n") {
                    // Create hashmap of values in string/2
                    let first_string = &line[0..line.len()/2];
                    let second_string = &line[line.len()/2..line.len()];

                   
                    for character in first_string.chars() {
                        if second_string.contains(character) {
                            sum += get_priority(character);
                            break;
                        }
                    }
                }
            }

            Err(_err) => panic!("Encountered error in reading file")
        } 
        sum
    }


    fn collect_sum(x: &str) -> u32 {
        let mut sum: u32 = 0; // Final Sum returned 

        // unwrap or just takes the Ok value or throws an error
        let line_as_string = fs::read_to_string(x).unwrap_or("Error reading file to string".to_owned());
        // Did into iterator because we need to go every three lines
        let mut string_iter = line_as_string.split("\n").into_iter();

        while let Some(first_string) = string_iter.next() {
            let second_string = string_iter.next().unwrap_or("Error unwrapping");
            let third_string = string_iter.next().unwrap_or("Error unwrapping");

            for character in first_string.chars() {
                if second_string.contains(character) && third_string.contains(character) {
                    sum += get_priority(character);
                    break;
                }
            }
        }

        sum
    }
}

fn main() {
    let j = Solution::sum("./src/data.txt");
    let j2 = Solution::collect_sum("./src/data.txt");
    println!("{}", j);
    println!("{}", j2);

    // println!("{}", get_priority('A'));
}
