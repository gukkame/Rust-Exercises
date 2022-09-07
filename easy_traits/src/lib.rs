#[derive(Clone, Debug)]
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
        self.value = self.value.replace(&[',', '.', '?', '!'], "");
        self
    }
}
