use std::collections::HashMap;
use std::fs;


fn main() {
    let mut point_map: HashMap<(&str, &str), u8> = HashMap::new();
    /*
        Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected 
        (1 for Rock, 2 for Paper, and 3 for Scissors) 
        plus the score for the outcome of the round 
        (0 if you lost, 3 if the round was a draw, and 6 if you won).


        PART 1
    */
    /*
    point_map.insert(("A", "X"), 4);
    point_map.insert(("A", "Y"), 8);
    point_map.insert(("A", "Z"), 3);

    point_map.insert(("B", "X"), 1);
    point_map.insert(("B", "Y"), 5);
    point_map.insert(("B", "Z"), 9);

    point_map.insert(("C", "X"), 7);
    point_map.insert(("C", "Y"), 2);
    point_map.insert(("C", "Z"), 6);
    */


    // Part 2
    point_map.insert(("A", "X"), 3);
    point_map.insert(("A", "Y"), 4);
    point_map.insert(("A", "Z"), 8);
    
    point_map.insert(("B", "X"), 1);
    point_map.insert(("B", "Y"), 5);
    point_map.insert(("B", "Z"), 9);

    point_map.insert(("C", "X"), 2);
    point_map.insert(("C", "Y"), 6);
    point_map.insert(("C", "Z"), 7);



    let mut total_points: i32 = 0;

    match fs::read_to_string("./src/data.txt") {
        Ok(line_as_string) => {
            for line in line_as_string.split("\n") {
                let  v: Vec<&str> = line.split_whitespace().collect();
                // println!("{} : {:?}", &v[0],  point_map.get(&(&v[0], &v[1])));

                match point_map.get(&(&v[0], &v[1])) {
                    Some(&value) => {
                        total_points += value as i32;

                    },

                    None => println!("Error getting value from dict")
                }

            }
        },
        Err(err) => {
            println!("Error: {}", err);
        }

    }

    println!("Total points: {}", total_points);
}


