use std::{fs, io::Error};

fn main() {
    if let Ok(list) = build_input() {
        let mut total: u32 = 0;
        for index in 0..list.0.len() {
            let current_point = list.0.get(index).unwrap();
            total += current_point * get_similarity_score(*current_point, &list.1);
        }
        println!("Total: {}", total);
    } else {
        println!("Could not build input");
    }
}

fn build_input() -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    let lines = fs::read_to_string("./inputs/input.txt")?;
    for line in lines.split("\n").into_iter() {
        let split_list: Vec<&str> = line.split("   ").collect();
        left_list.insert(left_list.len(),  split_list.get(0).unwrap().parse().unwrap());
        right_list.insert(right_list.len(),  split_list.get(1).unwrap().parse().unwrap());
    }
    Ok((left_list, right_list))
}

fn get_similarity_score(current_point: u32, right_list: &Vec<u32>) -> u32 {
    right_list.iter().filter(|&&item| item == current_point).count() as u32
}