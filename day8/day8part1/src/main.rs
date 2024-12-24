use std::{collections::{HashMap, HashSet}, fs, io::Error};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let file_name = "./inputs/input.txt";
    if let Ok(antenna_map) = build_antenna_map(file_name) {
        if let Ok(map_bounds) = get_map_bounds(file_name) {
            println!("{:#?}", antenna_map);
            let mut antinode_positions: HashSet<Point> = HashSet::new();
            for frequency in antenna_map.values() {
                let points = process_antenna_frequency(&frequency, map_bounds.0, map_bounds.1);
                for point in points {
                    antinode_positions.insert(point);
                }
            }
            println!("There are {} unique antinodes", antinode_positions.len());
        }
    }
}

fn build_antenna_map(file_name: &str) -> Result<HashMap<char, Vec<Point>>, Error> {
    let mut antenna_map: HashMap<char, Vec<Point>> = HashMap::new();
    for (row_index, line) in fs::read_to_string(file_name)?
        .split('\n')
        .into_iter()
        .enumerate()
    {
        for (column_index, char) in line.chars().enumerate() {
            if char != '.' {
                let val = antenna_map.get_mut(&char);
                if val.is_none() {
                    antenna_map.insert(char, vec![Point { x: column_index, y: row_index}]); 
                } else {
                    let len = val.as_ref().unwrap().len();
                    val.unwrap().insert(len, Point { x: column_index, y: row_index });
                }
            }
        }
    }
    return Ok(antenna_map);
}

fn get_map_bounds(file_name: &str) -> Result<(usize, usize), Error> {
    let binding = fs::read_to_string(file_name)?;
    let file: Vec<&str> = binding.split("\n").collect();
    let max_y = file.clone().first().unwrap().len();
    let max_x = file.first().unwrap().chars().count();
    return Ok((max_x, max_y));
    
}

fn process_antenna_frequency(antenntas: &Vec<Point>, max_x: usize, max_y: usize) -> HashSet<Point> {
    let mut antinode_positions: HashSet<Point> = HashSet::new();
    for current_antenna in antenntas {
        for other_antenna in antenntas {
            if current_antenna == other_antenna {
                continue;
            }
            let x_difference = current_antenna.x as i32 - other_antenna.x as i32;
            let y_difference = current_antenna.y as i32 - other_antenna.y as i32;
            
            let new_x = current_antenna.x as i32 + x_difference;
            let new_y = current_antenna.y as i32 + y_difference;

            if new_x < 0 || new_y < 0 || new_x >= max_x.try_into().unwrap() || new_y >= max_y.try_into().unwrap() {
                continue;
            }

            let antinode_point = Point { x: new_x as usize , y: new_y as usize };
            antinode_positions.insert(antinode_point);
        }
    }
    return antinode_positions;
}
