#[derive(Clone)]
pub struct StringValue {
     pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, new_str: String) -> &mut Self;

    fn append_number(&mut self, new_number: f64) -> &mut Self;

    fn remove_punctuation_marks(&mut self) -> &mut Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, new_str: String) -> &mut Self {
        let mut res = self.value.to_string();
        res.push_str(new_str.as_str());
        self.value = res;
        self
    }

    fn append_number(&mut self, new_number: f64) -> &mut Self {
        self.value += &new_number.to_string();
        self
    }

    fn remove_punctuation_marks(&mut self) -> &mut Self {
        let res = self.value.clone();
        let res2 = res.replace(|c: char| c.is_ascii_punctuation(), "");
        self.value = res2;
        self
    }
}
