use std::{fs, io::Error};

fn main() {
    if let Ok(input) = build_input() {
        let mut total = 0;
        for report in input {
            if is_report_difference_safe(&report) && (is_report_increasing(&report) || is_report_decreasing(&report)) {
                total += 1;
            }
        }
        println!("There are {} safe reports", total);
    }
}

fn build_input() -> Result<Vec<Vec<u32>>, Error> {
    let mut report_list: Vec<Vec<u32>> = vec![];

    let lines = fs::read_to_string("./inputs/input.txt")?;
    for line in lines.split("\n").into_iter() {
        let split_list: Vec<&str> = line.split(' ').collect();
        let mut report: Vec<u32> = vec![];
        for val in split_list {
            report.insert(report.len(), val.parse().unwrap());
        }
        report_list.insert(report_list.len(), report);
    }
    Ok(report_list)
}

fn is_report_difference_safe(line: &Vec<u32>) -> bool {
    line.windows(2)
        .map(|l| l.get(0).unwrap().abs_diff(*l.get(1).unwrap()))
        .into_iter()
        .all(|d| d <= 3 && d != 0)
}

fn is_report_increasing(line: &Vec<u32>) -> bool {
    line.windows(2)
        .into_iter()
        .all(|d| d.get(1).unwrap() > d.get(0).unwrap())
}

fn is_report_decreasing(line: &Vec<u32>) -> bool {
    line.windows(2)
        .into_iter()
        .all(|d| d.get(0).unwrap() > d.get(1).unwrap())
}