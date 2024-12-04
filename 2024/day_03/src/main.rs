use std::{fs::File, io::Read, io::Result};
use regex::Regex;



fn part_one(input: &str) -> u32 {
    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    reg.captures_iter(input)
        .filter_map(|cap| {
            match (cap.get(1), cap.get(2)) {
                (Some(x), Some(y)) => {
                    let x_val: u32 = x.as_str().parse().unwrap();
                    let y_val: u32 = y.as_str().parse().unwrap();
                    Some(x_val * y_val)
                },
                _ => None
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let reg = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut sum: u32 = 0;
    reg.captures_iter(input).for_each(|cap| { 
        let instruction = cap[0].to_string();
        if instruction == "do()" {
            enabled = true;
        }
        else if instruction == "don't()"{
            enabled = false
        }
        else{
            if enabled {
                sum += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
            }
        }
    });
    sum
}


fn main() -> Result<()>{
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    
    println!("Part one: {}", part_one(&contents));
    println!("Part two: {}", part_two(&contents));
    Ok(())
}
