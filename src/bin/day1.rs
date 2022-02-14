use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("PART 1");
    println!("======");
    println!("Test: {}", day1("data/day1_test.txt", 1).unwrap());
    println!("Puzzle: {}\n", day1("data/day1_puzzle.txt", 1).unwrap());
    println!("======");
    println!("PART 2");
    println!("======");
    println!("Test: {}", day1("data/day1_test.txt", 3).unwrap());
    println!("Puzzle: {}\n", day1("data/day1_puzzle.txt", 3).unwrap());
    Ok(())
}

fn day1(path: &str, size: usize) -> Result<u32, Error> {
    let buffer = BufReader::new(File::open(path)?);
    // Read all lines, convert all to u32
    let numbers: Vec<u32> = buffer
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    // Filter - functional
    let count = numbers
        .windows(size)
        .zip(numbers.windows(size).skip(1))
        .filter(|(a, b)| a.iter().sum::<u32>() < b.iter().sum::<u32>())
        .count();

    Ok(count.try_into().unwrap())

    // Filter - iterative
    // let mut count = 0;
    // for (a, b) in numbers.windows(size).zip(numbers.windows(size).skip(1)) {
    //     if &a.iter().sum::<u32>() < &b.iter().sum::<u32>() {
    //         count = count + 1;
    //     }
    // }
    // Ok(count)
}

// Key takeaways:
// zip() to bind two iterator into 1 of tuple pair
// windows() to slice a window
// turbo fish form as function::<T>()
// parse() only work for string