#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(self, new_str: String) -> Self;

    fn append_number(self, new_number: f64) -> Self;

    fn remove_punctuation_marks(self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(self, new_str: String) -> Self  {
       let mut res =  self.value.clone();
       res.push_str(new_str.as_str());
       StringValue { value: res }
    }

    fn append_number(self, new_number: f64) -> Self {
        let mut res =  self.value.clone();
        res.push_str(new_number.to_string().as_str());
       StringValue { value: res }
    }

    fn remove_punctuation_marks(self) -> Self {
        let res =  self.value.clone();
       let res2 = res.replace(|c: char| !c.is_ascii(), "");
       StringValue { value: res2 }
    }
}


