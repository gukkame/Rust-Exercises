use std::error::Error;
use std::fmt;
use std::fmt::Display;


#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Fail to parses todo")
    }
}
#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Fail to read todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let ParseErr::Empty = self {
            None
        } else {
            Some(self)
        }
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return Some(self.child_err.as_ref());
    }
}