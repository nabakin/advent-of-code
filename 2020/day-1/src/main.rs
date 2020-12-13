 use std::fs::File;
 use std::io::{BufRead, BufReader};
 use std::path::Path;
 use std::cmp::Ordering;
 
 fn main() {
    let mut expenses: Vec<u32> = file_lines_to_u32s(Path::new("2020/day-1/input.txt"));
    expenses.sort_unstable();

    let mut low_index = 0;
    let mut high_index = expenses.len()-1;

    while low_index < high_index {
        let current_sum = expenses[low_index] + expenses[high_index];
        
        match current_sum.partial_cmp(&2020).unwrap() {
            Ordering::Equal => {
                println!("{}", expenses[low_index] * expenses[high_index]);
                break;
            },
            Ordering::Less => low_index += 1,
            Ordering::Greater => high_index -= 1
        }
    }
}

fn file_lines_to_u32s(path: &Path) -> Vec<u32> {
    let input_file = File::open(path).unwrap();
    let reader = BufReader::new(&input_file);

    reader.lines()
        .map(|number_string| number_string.unwrap().parse::<u32>().unwrap())
        .collect()
}
