use std::
{
    fs::File,
    path::Path,
    io::{self, BufRead, Result, Lines, BufReader}
};


fn parse(lines: Lines<BufReader<File>>) -> Vec<Vec<u32>> {
    
    let mut reports: Vec<Vec<u32>> = Vec::new();

    for line in lines.flatten() {
        let report = 
            line.split(" ")
                .map(|x| x.parse().expect("Failed to parse."))
                .collect();
        reports.push(report);
    }
    reports
}

fn is_safe(report: Vec<u32>) -> bool {
     if report.len() <= 1 {
        return true;
    }

    let is_decreasing = report[0] > report[1];

    for window in report.windows(2) {
        let diff = window[0].abs_diff(window[1]);
    
        if diff < 1 || diff > 3 {
            return false;
        }

        if is_decreasing && window[0] < window[1] {
            return false;
        }
        
        if !is_decreasing && window[0] > window[1] {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(report: Vec<u32>) -> bool {
    
    if report.len() <= 2 {
        return true;
    }

    for i in 0..report.len() {
        
        let mut modified_report = report.to_vec();
        modified_report.remove(i);

        if is_safe(modified_report) {
            return true;
        }
    }

    false
}

fn part_one(reports: Vec<Vec<u32>>) -> u32{
    let mut safe_count: u32 = 0;
    for report in reports {
        if is_safe(report) {
            safe_count += 1
        }
    }
    return safe_count
}

fn part_two(reports: Vec<Vec<u32>>) -> u32{
    let mut safe_count: u32 = 0;
    for report in reports {
        if is_safe_with_dampener(report) {
            safe_count += 1
        }
    }
    return safe_count
}

fn main(){
    let lines = read_lines("input.txt").unwrap();

    let parsed_data = parse(lines);

    println!("Number of safe reports: {}", part_one(parsed_data.clone()));

    println!("Number of safe reports with dampener: {}", part_two(parsed_data.clone()))
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
