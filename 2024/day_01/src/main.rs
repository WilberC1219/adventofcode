use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // File hosts.txt must exist in the current path
    let mut list_one: Vec<u32> = Vec::new();
    let mut list_two: Vec<u32> = Vec::new();

    let lines = read_lines("input.txt").unwrap();

    // Consumes the iterator, returns an (Optional) String
    for line in lines.flatten() {
        // split the string by separator "   "
        let [val1, val2]: [u32; 2] = 
            line.split("   ")
                .map(|s| s.trim().parse().expect(&format!("Invalid input - failed to parse {}", s)))
                .collect::<Vec<u32>>()
                .try_into()
                .expect(&format!("Invalid input - row does not contain two values. {}", line));
    
        list_one.push(val1);
        list_two.push(val2);
    }
    // sort the lists
    list_one.sort();
    list_two.sort();

    // part one
    let mut sum: u32 = 0;        
    for (val1, val2) in list_one.iter().zip(list_two.iter()){
        sum += val1.abs_diff(*val2);
    }
    
    println!("The sum is {}", sum);

    // part two
    let mut freq: HashMap<u32, u32> = HashMap::new();
    for n in list_two {
        *freq.entry(n).or_default() += 1;
    }

    list_one.dedup();
    let mut sum:u32 = 0;
    for n in list_one {
        sum += n * freq.get(&n).copied().unwrap_or(0)
    }
    println!("The similarity score is {}", sum);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}