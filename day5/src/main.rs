use std::{io::{self, Write}, vec, fs};

struct Solution;

impl Solution {
    fn part1(vector: &mut Vec<Vec<char>>) {
        let read_str = fs::read_to_string("src/data.txt").unwrap_or("".to_owned());
        for line in read_str.split("\n") {
            let mut parts: Vec<usize> = vec![];

            line.split_whitespace()
            .for_each(|x| match x.parse::<usize>() {
                Ok(value) => {
                    parts.push(value);
                }
                _ => {}
            });


            // What index to start taking from
            // Also its stated above it because we won't be able to access the vector in this scope after we take the reference in the next line
            let starting_index = vector[parts[1] - 1].len() - parts[0];
            let to_take_from = &mut vector[parts[1] - 1];
            let elements = &mut to_take_from.drain(starting_index ..).collect::<Vec<_>>();

            elements.reverse(); // Crates are supoposed to be put 1 by 1, this kind of simulates that

            vector[parts[2] - 1].append(elements); // Append to end of the arg     
        }
    }

    fn part2(vector: &mut Vec<Vec<char>>) {
        let read_str = fs::read_to_string("src/data.txt").unwrap_or("".to_owned());
        for line in read_str.split("\n") {
            let mut parts: Vec<usize> = vec![];

            line.split_whitespace()
            .for_each(|x| match x.parse::<usize>() {
                Ok(value) => {
                    parts.push(value);
                }
                _ => {}
            });

            


            // What index to start taking from
            // Also its stated above it because we won't be able to access the vector in this scope after we take the reference in the next line
            let starting_index = vector[parts[1] - 1].len() - parts[0];
            let to_take_from = &mut vector[parts[1] - 1];
            let elements = &mut to_take_from.drain(starting_index ..).collect::<Vec<_>>();


            vector[parts[2] - 1].append(elements); // Append to end of the arg   
            

            Table::print_table(&vector);
            println!("{}", line);
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
        for i  in (1..=vector.len()) {
            print!("{}   ", i);
        }
        print!("\n");
    
        io::stdout().flush().unwrap();
    }
}


fn main() {
    // Create a stack for each row
    let mut main_vector = vec![
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


    // let mut test_vector = vec![
    //     vec!['Z', 'N'],
    //     vec!['M', 'C', 'D'],
    //     vec!['P']
    // ];


    // Table::print_table(&test_vector);
    // let j = Solution::part2(&mut test_vector);
    // Table::print_table(&test_vector);


    Table::print_table(&main_vector);
    let j = Solution::part2(&mut main_vector);
    Table::print_table(&main_vector);


}
