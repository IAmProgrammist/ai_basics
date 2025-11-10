use std::{fs::{self, File}, io::{BufRead, BufReader}};

use crate::utils::{BackpropElement, INPUT_NEURONS, OUTPUT_NEURONS};

pub const BACKPROP_OPERATIONS: [&str; 6] = ["and", "eq", "no", "or", "to", "xor"];
pub const BACKPROPS_OPERATIONS_LOCALIZATIONS: [&str; 6] = ["∧ - и", "≡ - равенство", "! - не", "∨ - или", "→ - следствие", "⊕ - исключающее или"];

pub fn load_dataset(wd: &str) -> Vec<BackpropElement> {
    let mut dataset: Vec<BackpropElement> = vec![];
    
    for (index, operation) in BACKPROP_OPERATIONS.iter().enumerate() {
        for path in fs::read_dir(format!("{wd}/{operation}")).unwrap() {
            if let Ok(file_path) = path {
                let mut backprop_element = BackpropElement {data_in: [0.; INPUT_NEURONS], out: [0.; OUTPUT_NEURONS]};
                backprop_element.out[index] = 1.;

                let file = File::open(file_path.path().to_str().unwrap_or("")).unwrap();
                let reader = BufReader::new(file);

                let mut actual_index: usize = 0;

                for line_result in reader.lines() {
                    if let Ok(line) = line_result {
                        for line_character in line.chars() {
                            if line_character == '1' {
                                backprop_element.data_in[actual_index] = 1.;
                                actual_index += 1;
                            } else if line_character == '0' {
                                actual_index += 1;
                            }
                        }
                    }
                }

                dataset.push(backprop_element);
            }
        }
    }

    dataset
}