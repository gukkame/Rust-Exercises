#[derive(Debug)]
#[derive(PartialEq)]
pub struct Student(pub i32, pub String, pub String);
pub fn id(student: &Student) -> i32 {
    let tup = student;
    tup.0
}

pub fn first_name(student: &Student) -> String {
    let tup = student;
    (*tup.1).to_string()
}

pub fn last_name(student: &Student) -> String {
    let tup = student;
    (*tup.2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let student = Student(20, String::from("Pedro"), String::from("Domingos"));
        assert_eq!(id(&student), 20);
    }
    #[test]
    fn test_first_name() {
        let student = Student(20, String::from("Pedro"), String::from("Domingos"));
        assert_eq!(first_name(&student), "Pedro".to_string());
    }
    #[test]
    fn test_last_name() {
        let student = Student(20, String::from("Pedro"), String::from("Domingos"));
        assert_eq!(last_name(&student), "Domingos".to_string());
    }
}
