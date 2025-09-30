use std::{error::Error, fs::File, io::{BufRead, BufReader}};

use crate::utils::ART1Database;

// Интерфейс-читалка базы данных
pub trait IART1DatabaseReader {
    fn read(self) -> Result<ART1Database, Box<dyn Error>>;
}

/* 
Читает базу данных из файла. Данные должны быть представлены в виде
1100110101
1100110101
1100110101
1100110101
1100110101
...
*/
pub struct FileART1DatabaseReader {
    file_path: String
}

impl FileART1DatabaseReader {
    pub fn new(file_path: &String) -> FileART1DatabaseReader {
        FileART1DatabaseReader {file_path: file_path.clone()}
    }
}

impl IART1DatabaseReader for FileART1DatabaseReader {
    fn read(self) -> Result<ART1Database, Box<dyn Error>> {
        // Открыть файл, инициализировать базу данных
        let file = File::open(self.file_path.as_str())?;
        let mut reader = BufReader::new(file);
        let mut result_database = ART1Database {dimension: 0, dataset: vec![]};
        
        // Из первой строчки мы должны определить количество свойств и считать первый элемент
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        result_database.dimension = line.trim().len();
        result_database.dataset.push(u64::from_str_radix(line.trim(), 2)?);

        for line in reader.lines() {
            let line_moved = line?;
            if line_moved.trim().len() == 0 {
                break;
            }
            result_database.dataset.push(u64::from_str_radix(line_moved.trim(), 2)?);    
        }

        Ok(result_database)
    }
}