use std::{fs, io::Error};

const VALID_XMAS_STRINGS: &[&str] = &["MSSM", "SMMS", "MMSS", "SSMM"];

fn main() {
    if let Ok(word_search) = build_input() {
        let mut total_xmas_count = 0;
        for (row_index, row) in word_search.clone().into_iter().enumerate() {
            for (column_index, character) in row.into_iter().enumerate() {
                if character == 'A' && check_a_value(row_index, column_index, &word_search) {
                    total_xmas_count += 1;
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

fn check_a_value(row_index: usize, column_index: usize, word_search: &Vec<Vec<char>>) -> bool {
    if row_index == 0 || column_index == 0 || row_index == word_search.len() - 1 || column_index == word_search[row_index].len() - 1 {
        return false
    }

    let mut surrounding_chars: [char; 4] = [' '; 4];
    surrounding_chars[0] = word_search[row_index - 1][column_index - 1];
    surrounding_chars[1] = word_search[row_index - 1][column_index + 1];
    surrounding_chars[2] = word_search[row_index + 1][column_index + 1];
    surrounding_chars[3] = word_search[row_index + 1][column_index - 1];

    let str: String = surrounding_chars.iter().collect();

    return VALID_XMAS_STRINGS.iter().any(|&xmas| xmas == str);
}

