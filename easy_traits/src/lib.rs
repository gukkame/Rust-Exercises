#[derive(Clone)]
pub struct StringValue {
     pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, new_str: String) -> &Self;

    fn append_number(&mut self, new_number: f64) -> &Self;

    fn remove_punctuation_marks(&mut self) -> &Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, new_str: String) -> &Self {
        let mut res = self.value.to_string();
        res.push_str(new_str.as_str());
        self.value = res;
        self
    }

    fn append_number(&mut self, new_number: f64) -> &Self {
        let mut res = self.value.to_string();
        res.push_str(new_number.to_string().as_str());
        println!("NUMBER: {:?}, {:?}", res, new_number);
        self.value = res;
        self
    }

    fn remove_punctuation_marks(&mut self) -> &Self {
        let res = self.value.clone();
        let res2 = res.replace(|c: char| c.is_ascii_punctuation(), "");
        self.value = res2;
        self
    }
}
