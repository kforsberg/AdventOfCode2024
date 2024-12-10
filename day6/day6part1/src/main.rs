use std::{collections::HashSet, fs, io::Error};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let map = build_input().unwrap();
    let current_coordinate = get_starting_coordinates(&map).unwrap();
    let seen_paths = traverse_map(&map, current_coordinate);
    println!("There are {} unique positions", seen_paths.len() + 1);
}

fn build_input() -> Result<Vec<Vec<char>>, Error> {
    let input = fs::read_to_string("./inputs/input.txt")?;
    let mut results: Vec<Vec<char>> = vec![];
    for line in input.split('\n') {
        results.insert(results.len(), line.chars().collect());

    }
    return Ok(results);
}

fn get_starting_coordinates(map: &Vec<Vec<char>>) -> Result<(usize, usize), Error> {
    for (row_index, row) in map.into_iter().enumerate() {
        for (column_index, character) in row.into_iter().enumerate() {
            if *character == '^' {
                return Ok((row_index, column_index));
            }
        }
    }
    Err(Error::other("Could not get starting coordinate"))
}

fn traverse_map(map: &Vec<Vec<char>>, starting_coordinate: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut seen_paths: HashSet<(usize, usize)> = HashSet::new();
    let mut current_direction: Direction = Direction::Up;
    let mut current_coordinate = Some(starting_coordinate.clone());
    
    loop {
        let next_coordinate = get_next_coordinate(current_coordinate.unwrap(), current_direction.clone(), map);
        if next_coordinate.is_none() {
            break;
        }
        if map[next_coordinate.unwrap().0][next_coordinate.unwrap().1] == '#' {
            current_direction = get_next_direction(current_direction);
        } else {
            seen_paths.insert(current_coordinate.unwrap());
            current_coordinate = next_coordinate;
        }
    }
    return seen_paths;
}

fn get_next_coordinate(current_coordinate: (usize, usize), current_direction: Direction, map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    if current_direction == Direction::Up && current_coordinate.0 > 0 {
        return Some((current_coordinate.0 - 1, current_coordinate.1 ))
    } else if current_direction == Direction::Down && current_coordinate.0 < map.len() - 1 {
        return Some((current_coordinate.0 + 1, current_coordinate.1))
    } else if current_direction == Direction::Left && current_coordinate.1 > 0 {
        return Some((current_coordinate.0, current_coordinate.1 - 1))
    } else if current_direction == Direction::Right && current_coordinate.1 < map.get(0).unwrap().len() - 1 {
        return Some((current_coordinate.0, current_coordinate.1 + 1))
    }
    None
}

fn get_next_direction(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up
    }
}