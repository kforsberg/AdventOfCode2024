use std::{fs, io::Error};

fn main() {
    if let Ok(word_search) = build_input() {
        let mut total_xmas_count = 0;
        for (row_index, row) in word_search.clone().into_iter().enumerate() {
            for (column_index, character) in row.into_iter().enumerate() {
                if character == 'X' {
                    total_xmas_count += check_x_value(row_index, column_index, &word_search);
                }
            }
        }
        println!("Total XMAS: {}", total_xmas_count);
    }
}

fn build_input() -> Result<Vec<Vec<char>>, Error> {
    let mut word_search: Vec<Vec<char>> = vec![];

    let lines = fs::read_to_string("./inputs/input.txt")?;
    for line in lines.split("\n").into_iter() {
        let mut word_search_line: Vec<char> = vec![];
        for val in line.chars() {
            word_search_line.insert(word_search_line.len(), val);
        }
        word_search.insert(word_search.len(), word_search_line);
    }
    Ok(word_search)
}

fn check_x_value(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> u32 {
    let mut total_matches = 0;
    if check_up(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_up_right(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_right(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_down_right(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_down(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_down_left(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_left(row_index, column_index, word_search) {
        total_matches += 1;
    }
    if check_up_left(row_index, column_index, word_search) {
        total_matches += 1;
    }

    total_matches
}

fn check_up(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index < 3 {
        return false;
    }

    return word_search[row_index - 1][column_index] == 'M'
        && word_search[row_index - 2][column_index] == 'A'
        && word_search[row_index - 3][column_index] == 'S';
}

fn check_up_right(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index < 3 || column_index >= word_search[row_index].len() - 3 {
        return false;
    }

    return word_search[row_index - 1][column_index + 1] == 'M'
        && word_search[row_index - 2][column_index + 2] == 'A'
        && word_search[row_index - 3][column_index + 3] == 'S';
}

fn check_right(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if column_index >= word_search[row_index].len() - 3 {
        return false;
    }

    return word_search[row_index][column_index + 1] == 'M'
        && word_search[row_index][column_index + 2] == 'A'
        && word_search[row_index][column_index + 3] == 'S';
}

fn check_down_right(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index >= word_search.len() - 3 || column_index >= word_search[row_index].len() - 3 {
        return false;
    }

    return word_search[row_index + 1][column_index + 1] == 'M'
        && word_search[row_index + 2][column_index + 2] == 'A'
        && word_search[row_index + 3][column_index + 3] == 'S';
}

fn check_down(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index >= word_search.len() - 3 {
        return false;
    }

    return word_search[row_index + 1][column_index] == 'M'
        && word_search[row_index + 2][column_index] == 'A'
        && word_search[row_index + 3][column_index] == 'S';
}

fn check_down_left(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index >= word_search.len() - 3 || column_index < 3 {
        return false;
    }

    return word_search[row_index + 1][column_index - 1] == 'M'
        && word_search[row_index + 2][column_index - 2] == 'A'
        && word_search[row_index + 3][column_index - 3] == 'S';
}

fn check_left(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if column_index < 3 {
        return false;
    }

    return word_search[row_index][column_index - 1] == 'M'
        && word_search[row_index][column_index - 2] == 'A'
        && word_search[row_index][column_index - 3] == 'S';
}

fn check_up_left(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index < 3 || column_index < 3 {
        return false;
    }

    return word_search[row_index - 1][column_index - 1] == 'M'
        && word_search[row_index - 2][column_index - 2] == 'A'
        && word_search[row_index - 3][column_index - 3] == 'S';
}
