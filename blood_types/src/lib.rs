pub use std::cmp::{Ord, Ordering};
pub use std::error::Error;
pub use core::fmt::Formatter;
pub use std::fmt::{self, Debug};
pub use std::str::FromStr;
pub use std::num::ParseIntError;
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}
#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}
#[derive(Debug)]
pub struct ParseError {
    pub details: String,
}
impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
impl FromStr for Antigen {
    type Err = ParseError;
    fn from_str(anti: &str) -> Result<Self, Self::Err> {
        match anti {
            "A"  => {return Ok(Antigen::A)}, 
            "B"  => {return Ok(Antigen::B)},
            "AB" => {return Ok(Antigen::AB)}
            "O"  => {return Ok(Antigen::O)},
            _    => {return Err(ParseError::new("Antigen in wrong format"))}
        }
    }
}
impl FromStr for RhFactor {
    type Err = ParseError;
    fn from_str(rh: &str) -> Result<Self, Self::Err> {
        match rh {
            "-" => {return Ok(RhFactor::Negative)},
            "+" => {return Ok(RhFactor::Positive)},
            _   => {return Err(ParseError::new("RhFactor in wrong format"))}
        }
    }
}
impl FromStr for BloodType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_at(s.len()-1);
        let antigen: Antigen = match parts.0.parse() {
            Ok(resp) => resp,
            Err(e) => {return Err(e)}
        };
        let rh_factor: RhFactor = match parts.1.parse() {
            Ok(resp) => resp,
            Err(e) => {return Err(e)}
        };
        Ok(BloodType {
            antigen,
            rh_factor
        })
    }
}
impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        match &self.antigen {
            Antigen::A => {
                if other.antigen == Antigen::B || 
                   self.antigen == other.antigen && other.rh_factor == RhFactor::Negative {
                    return Ordering::Greater;
                }
                return Ordering::Less
            }
            Antigen::B => {
                if other.antigen == self.antigen && self.rh_factor == RhFactor::Positive {
                    return Ordering::Greater
                } else {
                    return Ordering::Less
                }
            }
            Antigen::AB => {
                if other.antigen == self.antigen && self.rh_factor == RhFactor::Negative {
                    return Ordering::Less
                } else {
                    return Ordering::Greater
                }
            }
            Antigen::O => {
                if other.antigen == Antigen::AB || 
                   self.antigen == other.antigen && other.rh_factor == RhFactor::Positive {
                    return Ordering::Less;
                }
                return Ordering::Greater
            }
        }
    }
}
impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        match self.antigen {
            Antigen::A => {string += "A"},
            Antigen::B => {string += "B"},
            Antigen::AB => {string += "AB"},
            Antigen::O => {string += "O"},
        }
        match self.rh_factor {
            RhFactor::Negative => {string += "-"},
            RhFactor::Positive => {string += "+"},
        }
        f.debug_struct(string.as_str()).finish()
    }
}
impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        match self.rh_factor {
            RhFactor::Negative => {
                if other.rh_factor == RhFactor::Positive {
                    return false
                }
            },
            _ => {}
        }
        match self.antigen {
            Antigen::A => {
                match other.antigen {
                    Antigen::B | Antigen::AB => {return false},
                    _=> {}
                }
                
            },
            Antigen::B => {
                match other.antigen {
                    Antigen::A | Antigen::AB => {return false},
                    _=> {}
                }
            },
            Antigen::O => {
                match other.antigen {
                    Antigen::A | Antigen::B | Antigen::AB => {return false},
                    _=> {}
                }
            },
            _ => {},
        }
        return true
    }
	pub fn donors(&self) -> Vec<Self> {
        let mut arr: Vec<BloodType> = Vec::new();
        match self.antigen {
            Antigen::A => {
                arr.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative
                });
                arr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative
                });
            },
            Antigen::B => {
                arr.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative
                });
                arr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative
                });
            },
            Antigen::AB => {
                arr.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Negative
                });
                arr.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative
                });
                arr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative
                });
                arr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative
                });
            },
            Antigen::O => {
                arr.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative
                });
            }
        }
        if self.rh_factor == RhFactor::Positive {
            for mut blood in arr.clone() {
                blood.rh_factor = RhFactor::Positive;
                arr.push(blood)
            }
        }
        return arr
	}
    
	pub fn recipients(&self) -> Vec<BloodType> {
        let mut arr: Vec<BloodType> = Vec::new();
    
        match self.antigen {
            Antigen::A  => {
                arr.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
                arr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive
                });
            },
            Antigen::B  => {
                arr.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
                arr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive
                });
            },
            Antigen::AB => {
                arr.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
            },
            Antigen::O  => {
                arr.push(BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive
                });
                arr.push(BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Positive
                });
                arr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive
                });
                arr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive
                });
            }
        }
    
        if self.rh_factor == RhFactor::Negative {
            for mut blood in arr.clone() {
                blood.rh_factor = RhFactor::Negative;
                arr.push(blood)
            }
        }
    
        return arr
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn compatible_ab_neg_with_a_pos() {
	let blood_type: BloodType = "AB-".parse().unwrap();
	let other_bt: BloodType = "A+".parse().unwrap();
	assert!(!blood_type.can_receive_from(&other_bt));
}
#[test]
fn compatible_a_neg_with_a_pos() {
	let blood_type: BloodType = "A-".parse().unwrap();
	let other_bt: BloodType = "A+".parse().unwrap();
	assert!(!blood_type.can_receive_from(&other_bt));
}
#[test]
fn compatible_a_neg_with_ab_neg() {
	let blood_type: BloodType = "AB-".parse().unwrap();
	let other_bt: BloodType = "A-".parse().unwrap();
	assert!(blood_type.can_receive_from(&other_bt));
}
#[test]
fn compatible_ab_neg_with_o_pos() {
	let blood_type: BloodType = "AB-".parse().unwrap();
	let other_bt: BloodType = "O+".parse().unwrap();
	assert!(!blood_type.can_receive_from(&other_bt));
}
#[test]
fn compatible_ab_pos_with_o_pos() {
	let blood_type: BloodType = "AB+".parse().unwrap();
	let other_bt: BloodType = "O+".parse().unwrap();
	assert!(blood_type.can_receive_from(&other_bt));
}
#[test]
fn test_compatible_ab_neg_with_o_neg() {
	let blood_type: BloodType = "AB-".parse().unwrap();
	let other_bt: BloodType = "O-".parse().unwrap();
	assert!(blood_type.can_receive_from(&other_bt));
}
#[test]
fn test_antigen_ab_from_str() {
	let blood = "AB+";
	let blood_type: BloodType = blood.parse().unwrap();
	assert_eq!(blood_type.antigen, Antigen::AB);
	assert_eq!(blood_type.rh_factor, RhFactor::Positive);
}
#[test]
fn test_antigen_a_from_str() {
	let blood = "A-";
	let blood_type = blood.parse::<BloodType>().unwrap();
	assert_eq!(blood_type.antigen, Antigen::A);
	assert_eq!(blood_type.rh_factor, RhFactor::Negative);
}
#[test]
#[should_panic]
fn test_unexistent_blood_type() {
	let _blood_type: BloodType = "AO-".parse().unwrap();
}
#[test]
fn test_donors() {
	let mut givers = "AB+".parse::<BloodType>().unwrap().donors();
	println!("Before sorting {:?}", &givers);
	givers.sort();
	println!("{:?}", &givers);
	let mut expected = vec![
		"AB-".parse::<BloodType>().unwrap(),
		"A-".parse().unwrap(),
		"B-".parse().unwrap(),
		"O-".parse().unwrap(),
		"AB+".parse().unwrap(),
		"A+".parse().unwrap(),
		"B+".parse().unwrap(),
		"O+".parse().unwrap(),
	];
	expected.sort();
	assert_eq!(givers, expected);
}
#[test]
fn test_a_neg_donors() {
	let mut givers = "A-".parse::<BloodType>().unwrap().donors();
	givers.sort();
	let mut expected: Vec<BloodType> = vec!["A-".parse().unwrap(), "O-".parse().unwrap()];
	expected.sort();
	assert_eq!(givers, expected);
}
#[test]
fn test_o_neg_donors() {
	let mut givers = "O-".parse::<BloodType>().unwrap().donors();
	givers.sort();
	let mut expected: Vec<BloodType> = vec!["O-".parse().unwrap()];
	expected.sort();
	assert_eq!(givers, expected);
}
#[test]
fn test_ab_pos_recipients() {
	let mut recipients: Vec<BloodType> = "AB+".parse::<BloodType>().unwrap().recipients();
	recipients.sort();
	let mut expected: Vec<BloodType> = vec!["AB+".parse().unwrap()];
	expected.sort();
	assert_eq!(recipients, expected);
}
#[test]
fn test_a_neg_recipients() {
	let mut recipients = "A-".parse::<BloodType>().unwrap().recipients();
	recipients.sort();
	let mut expected: Vec<BloodType> = vec![
		"A-".parse().unwrap(),
		"AB+".parse().unwrap(),
		"A+".parse().unwrap(),
		"AB-".parse().unwrap(),
	];
	expected.sort();
	assert_eq!(recipients, expected);
}
#[test]
fn test_output() {
	let blood_type: BloodType = "O+".parse().unwrap();
	println!("recipients of O+ {:?}", blood_type.recipients());
	println!("donors of O+ {:?}", blood_type.donors());
	let another_blood_type: BloodType = "A-".parse().unwrap();
	println!(
		"donors of O+ can receive from {:?} {:?}",
		&another_blood_type,
		blood_type.can_receive_from(&another_blood_type)
	);
}
}
