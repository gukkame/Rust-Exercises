#[warn(dead_code)]
pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content == "" {
            None
        } else {
            let words = self.content.split_whitespace();
            for word in words {
                if word == "stupid" {
                    return None;
                }
            }
            return Some(&self.content);
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    let res = Message::send_ms(ms);
    if res == None {
        (false, "ERROR: illegal")
    } else {
        (true, &ms.content)
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_ms() {
        let v = vec![
            Message::new("".to_string(), "toby".to_string()),
            Message::new("stupid".to_string(), "jack".to_string()),
            Message::new("you are stupid".to_string(), "jacob".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(!t);
        }
    }
    #[test]
    fn test_ok_ms() {
        let v = vec![
            Message::new("get out of the car".to_string(), "police".to_string()),
            Message::new("no!".to_string(), "thief".to_string()),
            Message::new("get the werewolf".to_string(), "police".to_string()),
            Message::new("wait the wha...".to_string(), "thief".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(t);
        }
    }
}
