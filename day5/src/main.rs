use std::{io::{self, Write}, vec, fs};

struct Solution;

impl Solution {
    fn part1(vector: &Vec<Vec<char>>) {

        let read_str = fs::read_to_string("src/data.txt").unwrap_or("".to_owned());
        for line in read_str.split("\n") {
            let mut parts: Vec<u32> = vec![];

            line.split_whitespace()
            .for_each(|x| match x.parse::<u32>() {
                Ok(value) => {
                    parts.push(value);
                }
                _ => {}
            });

            // vec.Truncate
            // vec.drain
            
            
            
            
        }
    }
}


struct Table;

impl Table {
    fn get_len(vector: &Vec<Vec<char>>) -> usize {
        let mut len: usize = 0;
    
        for sub_array in vector {
            if sub_array.len() > len {
                len = sub_array.len();
            }
        }
    
        len
    }
    
    fn print_table(vector: &Vec<Vec<char>>) {
    
        let len = Table::get_len(vector);
    
        for i in (0..len).rev() {
            for column in vector.into_iter() {
                match column.get(i) {
                    Some(value) => {
                        print!("[{}] ", value);
                    },
                    None => {
                        print!("    "); // 4 whitespace chars
                    }
                }
            }
    
            print!("\n");
            
        }
        print!(" ");
        for i  in (1..=len+1) {
            print!("{}   ", i);
        }
        print!("\n");
    
        io::stdout().flush().unwrap();
    }
}


fn main() {
    // Create a stack for each row
    let main_vector = vec![
        vec!['R', 'G', 'H', 'Q', 'S', 'B', 'T', 'N'],
        vec!['H', 'S', 'F', 'D', 'P', 'Z', 'J'],
        vec!['Z', 'H', 'V'],
        vec!['M', 'Z', 'J', 'F', 'G', 'H'],
        vec!['T', 'Z', 'C', 'D', 'L', 'M', 'S', 'R'],
        vec!['M', 'T', 'W', 'V', 'H', 'Z', 'J'],
        vec!['T', 'F', 'P', 'L', 'Z'],
        vec!['Q', 'V', 'W', 'S'],
        vec!['W', 'H', 'L', 'M', 'T', 'D', 'N', 'C']
    ];

    Table::print_table(&main_vector);
    let j = Solution::part1(&main_vector);
}
