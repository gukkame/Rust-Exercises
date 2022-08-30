pub use chrono::{NaiveDate, Utc};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FErr {
    pub fn new(name: String, error: String, err: String) -> FErr {
        FErr {
            form_values: (name, error),
            date: "".to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_colour: Color,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_colour: Color,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name: first_name,
            last_name: last_name,
            birth: birth,
            fav_colour: fav_colour,
            birth_location: birth_location,
            password: password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FErr> {
        let mut vec: Vec<&str> = Vec::<&str>::new();
        let f_n = self.first_name.to_string();
        let pass = self.password.to_string();
        let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();



        if self.first_name == "" {
            Err(FErr {
                form_values: ("first_name".to_string(), f_n),
                date: date,
                err: "No user name".to_string(),
            })
        } else if self.password.len() >= 8 {
            let mut has_number = false;
            let mut has_letters = false;
            let mut has_none_alphanumeric = false;
            for ch in self.password.chars() {
                if ch.is_ascii_digit() {
                    has_number = true;
                }
                if ch.is_alphabetic() {
                    has_letters = true;
                }
                if ch.is_ascii_punctuation() {
                    has_none_alphanumeric = true;
                }
            }
            if (has_number == false && has_letters == true && has_none_alphanumeric == false)
                || (has_number == true && has_letters == true && has_none_alphanumeric == false)
                || (has_number == false && has_letters == true && has_none_alphanumeric == true)
            {
                Err(FErr { form_values : ("password".to_string(), pass), date: date, err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string() })
            } else {
                vec.push("Valid first name");
                vec.push("Valid password");
                println!("VV: {:?}", vec);
                return Ok(vec);
            }
        } else {
            Err(FErr {
                form_values: ("password".to_string(), pass),
                date: date,
                err: "At least 8 characters".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug)]
    struct TestForm<'a> {
        form: Form,
        validation: Result<Vec<&'a str>, FErr>,
    }
    impl<'a> TestForm<'_> {
        // all test cases
        fn new() -> Vec<TestForm<'a>> {
            vec![
                TestForm {
                    form : Form::new(
                    String::from("Katy"),
                    String::from("Silva"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Ok(vec!["Valid first name", "Valid password"]),
                },
                TestForm {
                    form : Form::new(
                    String::from(""),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Err(FErr {
                        form_values: (String::from("first_name"),
                        String::from("")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("No user name")}),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("12345")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("12345")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("At least 8 characters") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("sdASDsrW")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("sdASDsrW")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("dsGE1SAD213")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("dsGE1SAD213")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("dsaSD&%DF!?=")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("dsaSD&%DF!?=")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                }
            ]
        }
    }
    #[test]
    fn test_error_type() {
        let form_cases = TestForm::new();
        for v in form_cases {
            assert_eq!(v.form.validate(), v.validation);
        }
    }
    #[allow(dead_code)]
    fn create_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    }
}
