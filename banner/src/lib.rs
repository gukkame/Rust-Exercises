use std::{collections::HashMap, num::ParseFloatError};
use std::str::FromStr;
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        
        let mut s_h = "".to_string();
        s_h.push('-');
        for (i, ch) in l_h.chars().enumerate() {
            if i == 2 {
                s_h.push(ch); 
            }
        }
        Flag { short_hand: s_h.to_string(), long_hand: l_h.to_string(), desc: d.to_string() }
        // let d = Flag::opt_flag("diff", "gives the difference between two numbers");

        // println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
        // // output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let vec = argv.clone();
        if flag.0 == "-d".to_string() {
            match div(vec[0],vec[1]){
                Err(_why) => "invalid float literal".to_string(),
                Ok(value) => value
            }
        }else{
            match rem(vec[0],vec[1]){
                Err(_why) => "invalid float literal".to_string(),
                Ok(value) => value
            }
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    if let Err(e) = f64::from_str(a) {
        return Err(e)
    }else if let Err(e) = f64::from_str(b) {
        return Err(e)
    }else {
        let c =  f32::from_str(a).unwrap();
        let v =  f32::from_str(b).unwrap();
         return Ok((c/v).to_string())
    }
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    if let Err(e) = f64::from_str(a) {
        return Err(e)
    }else if let Err(e) = f64::from_str(b) {
        return Err(e)
    }else {
        let c =  f32::from_str(a).unwrap();
        let v =  f32::from_str(b).unwrap();
         return Ok((c%v).to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn init() -> FlagsHandler {
        let d = Flag::opt_flag("division", "divides two numbers");
        let r = Flag::opt_flag(
            "remainder",
            "gives the remainder of the division between two numbers",
        );
        let mut handler = FlagsHandler { flags: HashMap::new() };
        handler.add_flag((d.short_hand, d.long_hand), div);
        handler.add_flag((r.short_hand, r.long_hand), rem);
        return handler;
    }
    #[test]
    fn ok_test() {
        let mut handler = init();
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "2.0"]),
            "0.5"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "2.0"]),
            "0"
        );
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["12.323", "212.32"]),
            "0.05803975"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["12.323", "212.32"]),
            "12.323"
        );
    }
    #[test]
    fn error_test() {
        let mut handler = init();
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]),
            "invalid float literal"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "f"]),
            "invalid float literal"
        );
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "0.0"]),
            "inf"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "0.0"]),
            "NaN"
        );
    }
}



