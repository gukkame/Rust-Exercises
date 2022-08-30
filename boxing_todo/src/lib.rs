mod err;
use err::{ParseErr, ReadErr};
use std::fs;
use std::io::prelude::*;
// use std::iter::Map;

pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut content = String::new();
        let file = fs::File::open(path);
        if let Err(a) = file {
                let x = ReadErr{child_err: a.into()};
                return Err(x.into())
        }

        let m = file.unwrap().read_to_string(&mut content);
        
        if m.is_err() {
            return Err(Box::new(ParseErr::Empty))
        };
        let i = parse(&content);
        if let Err(a) = i {
            let m = ParseErr::Malformed(a.into());
            return Err(Box::new(m))
        };
        let i = i.unwrap();
        let x = &i["tasks"];
        if x.is_empty() {
            return Err(Box::new(ParseErr::Empty))
        } 
        let mut tasks: Vec<Task> = vec![];
        for i in x.members() {
            tasks.push(Task {id: i["id"].as_u32().unwrap(), description: i["description"].to_string(), level: i["level"].as_u32().unwrap()})
        }
        Ok(TodoList {title: i["title"].to_string(), tasks})
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};
    use super::*;

    const CORRECT: &str = r#"{"title":"TODO TITLE","tasks":[{"id":0,"description":"do this","level":0},{"id":1,"description":"do that","level":5}]}"#;
    const MALFORMED: &str = r#"{"title":"TODO TITLE","tasks":[{"id":0,"description":"do this","level":0},{"id":1,"description":"do that","level":5},]}"#;
    const EMPTY: &str = r#"{"title":"TODO TITLE","tasks":[]}"#;

    #[test]
    fn open_test() {
        let err = TodoList::get_todo("file_does_not_exist.json").unwrap_err();

        assert_eq!("Fail to read todo file", err.to_string());

        let source: &io::Error = err.source().expect("Error doesn't have source")
            .downcast_ref().expect("Expected error to be io::Error");
        println!("{}", source);
        assert_eq!(io::ErrorKind::NotFound, source.kind(), "Error is not of the expected kind");
    }

    #[test]
    fn correct_test() {
        fs::write("test_correct.json", CORRECT).expect("Failed to write test file");

        let todos = TodoList::get_todo("test_correct.json").expect("Failed to get todos: ");

        assert_eq!("TODO TITLE", todos.title);

        let v = vec![
            Task {id: 0, description: "do this".to_string(), level: 0},
            Task {id: 1, description: "do that".to_string(), level: 5},
        ];
        assert_eq!(v, todos.tasks);

        fs::remove_file("test_correct.json").expect("Could not remove test file");
    }

    #[test]
    fn malformed_test() {
        fs::write("test_malformed.json", MALFORMED).expect("Failed to write test file");

        let err = TodoList::get_todo("test_malformed.json").expect_err("Shouldn't read invalid json");

        assert_eq!("Fail to parses todo", err.to_string()); // Typo here is intentional
        assert_eq!("Fail to parses todo", err.source().unwrap().to_string());

        fs::remove_file("test_malformed.json").expect("Could not remove test file");
    }

    #[test]
    fn empty_test() {
        fs::write("test_empty.json", EMPTY).expect("Failed to write test file");

        let err = TodoList::get_todo("test_empty.json").expect_err("Shouldn't read empty list");

        assert_eq!("Fail to parses todo", err.to_string()); // Typo here is intentional
        assert!(err.source().is_none());

        fs::remove_file("test_empty.json").expect("Could not remove test file");
    }
}