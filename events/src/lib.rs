pub use chrono::Duration;
pub use colored::*;
pub use Event::*;
pub use std::fmt;
#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}
#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}
#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}
impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content.truecolor(self.color.0, self.color.1, self.color.2))
    }
}
impl Event<'_> {
	pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string()
            },
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string()
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string()
            },
            Registration(time) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {}H:{}M:{}S left before the registration ends",
                    time.num_hours(),
                    time.num_minutes()%60,
                    time.num_seconds() -time.num_minutes()*60
                )
            }
        }
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn remainder_notification() {
		let remainder = Remainder("Go to the doctor");
		let notification = remainder.notify();
		println!("{}", &notification);
		assert_eq!(
			notification,
			Notification {
				size: 50,
				color: (50, 50, 50),
				position: Position::Bottom,
				content: "Go to the doctor".to_string(),
			}
		);
	}
	#[test]
	fn registration_notification() {
		let registration = Registration(Duration::seconds(49094));
		let notification = registration.notify();
		println!("{}", registration.notify());
		assert_eq!(
			notification,
			Notification {
				size: 30,
				color: (255, 2, 22),
				position: Position::Top,
				content: "You have 13H:38M:14S left before the registration ends".to_string(),
			}
		);
	}
	#[test]
	fn appointment_notification() {
		let appointment = Appointment("Go to the doctor");
		let notification = appointment.notify();
		println!("{}", &notification);
		assert_eq!(
			notification,
			Notification {
				size: 100,
				color: (200, 200, 3),
				position: Position::Center,
				content: "Go to the doctor".to_string(),
			}
		);
	}
	#[test]
	fn holiday_notification() {
		let holiday = Holiday;
		let notification = Holiday.notify();
		println!("{}", holiday.notify());
		assert_eq!(
			notification,
			Notification {
				size: 25,
				color: (0, 255, 0),
				position: Position::Top,
				content: String::from("Enjoy your holiday"),
			}
		);
	}
}