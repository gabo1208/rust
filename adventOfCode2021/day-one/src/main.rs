use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use queues::*;

fn main() {
    if let Ok(lines) = read_lines("./input-a.dat") {
        let mut count = 0;
        let mut previous = String::new();

        for line in lines {
            if let Ok(current) = line {
                if previous != "" && current.parse::<i32>().unwrap() > previous.parse::<i32>().unwrap() {
                    count += 1
                }
                previous = current
            }
        }

        println!("First part: {}", count);
    }

    if let Ok(lines) = read_lines("./input-a.dat") {
        let mut index = 0;
        let mut count = 0;
        let mut previous_sum: i32;
        let mut current_sum = 0;
        let mut window_queue: Queue<i32> = queue![];
        let mut first_n: i32;

        for line in lines {
            if let Ok(current) = line {
                index += 1;
                first_n = current.parse::<i32>().unwrap();
                if let Err(_e) = window_queue.add(first_n) {
                    println!("Error adding to the queue {}", count);
                }

                previous_sum = current_sum;
                current_sum += first_n;
                if index > 3 {
                    if let Ok(third_n) = window_queue.remove() {
                        current_sum -= third_n;
                        if current_sum > previous_sum {
                            count += 1
                        }
                    }
                }
            }
        }

        println!("Second part: {}", count);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}