use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Part1: {:?}", day3("input1.txt"));
    println!("Part1: {:?}", day3("input2.txt"));
    println!("Part1: {:?}", day3_2("input1.txt"));
    println!("Part1: {:?}", day3_2("input2.txt"));
    Ok(())
}

fn day3(path: &str) -> Result<u32, Error> {
    let buffer = BufReader::new(File::open(path)?);
    let bin_strings: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();

    let mut hm: HashMap<(usize, char), usize> = HashMap::new();
    for a_str in bin_strings.iter() {
        for idx_char in a_str.chars().enumerate() {
            *hm.entry(idx_char).or_insert(0) += 1;
        }
    }

    let mut gamma_vec: Vec<char> = vec![];
    let mut epsilon_vec: Vec<char> = vec![];

    for idx in 0..(hm.len() / 2) {
        if hm.get(&(idx, '0')).unwrap() < hm.get(&(idx, '1')).unwrap() {
            gamma_vec.push('1');
            epsilon_vec.push('0');
        } else {
            gamma_vec.push('0');
            epsilon_vec.push('1');
        }
    }

    let gamma_str: String = gamma_vec.into_iter().collect();
    let epsilon_str: String = epsilon_vec.into_iter().collect();

    Ok(u32::from_str_radix(&gamma_str, 2).unwrap() * u32::from_str_radix(&epsilon_str, 2).unwrap())
}

fn day3_2(path: &str) -> Result<u32, Error> {
    let buffer = BufReader::new(File::open(path)?);
    let bin_strings: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();

    let length = bin_strings.first().unwrap().chars().count();
    let total_entries = bin_strings.len();
    println!("{}", length);

    let mut bin_strings_clone = bin_strings.clone();

    for idx in 0..length {
        let mut most = '1';
        let mut count = 0;
        for a_str in bin_strings_clone.iter() {
            if a_str.chars().nth(idx).unwrap() == '1' {
                count += 1;
            }
        }

        if count < total_entries/2 {
            most = '0';
        }

        

    }


    Ok(0)
}
