use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Part 1: {:?}", day2_part1("input1.txt").unwrap());
    println!("Part 2: {:?}", day2_part2("input2.txt").unwrap());

    Ok(())
}

fn day2_part1(path: &str) -> Result<u32, Error> {
    let buffer = BufReader::new(File::open(path)?);
    let instructions: Vec<String> = buffer.lines().map(|line| line.unwrap()).collect();

    let mut position = (0, 0);

    for unsplit in instructions.iter() {
        let split: Vec<_> = unsplit.split_whitespace().collect();
        let step_size = split.get(1).unwrap().parse::<u32>().unwrap();
        match split.get(0).unwrap().as_ref() {
            "forward" => {
                position = (position.0 + step_size, position.1);
            }
            "up" => {
                position = (position.0, position.1 - step_size);
            }
            "down" => {
                position = (position.0, position.1 + step_size);
            }
            _ => {}
        }
    }

    let product = position.0 * position.1;

    return Ok(product);
}

fn day2_part2(path: &str) -> Result<u32, Error> {
    let buffer = BufReader::new(File::open(path)?);
    let instructions: Vec<String> = buffer.lines().map(|line| line.unwrap()).collect();

    let mut position = (0, 0);
    let mut aim = 0;

    for unsplit in instructions.iter() {
        let split: Vec<_> = unsplit.split_whitespace().collect();
        let step_size = split.get(1).unwrap().parse::<u32>().unwrap();
        match split.get(0).unwrap().as_ref() {
            "forward" => {
                position = (position.0 + step_size, position.1 + (step_size * aim));
            }
            "up" => {
                aim = aim - step_size;
            }
            "down" => {
                aim = aim + step_size;
            }
            _ => {}
        }
    }

    let product = position.0 * position.1;

    return Ok(product);
}

// Takeaway:
// Allocate when doing Vec<> means we need to group as tuple or something?
// Cannot be built from std::iter::Iterator... then need watch see if the type match?
