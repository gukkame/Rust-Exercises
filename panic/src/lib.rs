// use panic::*;
use std::fs::{File};

pub fn open_file(s: &str) -> File {
    let file_result = File::open(s);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    file
    
}
// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	#[test]
// 	#[should_panic]
// 	fn test_opening() {
// 		open_file("file.txt");
// 	}
// 	#[test]
// 	fn test_opening_existing() {
// 		let filename = "created.txt";
// 		File::create(filename).unwrap();
// 		open_file(filename);
// 		fs::remove_file(filename).unwrap();
// 	}
// }
