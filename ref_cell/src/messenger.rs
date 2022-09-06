pub use std::rc::Rc;
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}
pub struct Tracker<'a, T:Logger> {
    pub logger: &'a T,
    pub value: usize,
    pub max: usize,
}
impl<'a, T> Tracker<'a, T> where T: Logger {
    pub fn new(logger: &'a T, max: usize) -> Tracker<'a, T> {
        return Tracker {
            logger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&self, v: &Rc<usize>){
        let value = Rc::strong_count(&v);
        let per = ((value as f32 / self.max as f32) *100.0).floor() as i32;
        if per >= 100 {
            self.logger.error("Error: you are over your quota!")
        } else if per >= 70 && per < 100 {
            self.logger.warning(
                format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", per)
                .as_str()
            )
        }
    }
    pub fn peek(&self, v: &Rc<usize>) {
        let value = Rc::strong_count(v);
        let per = ((value as f32 / self.max as f32) *100.0).floor() as i32;
        self.logger.info(
            format!("Info: you are using up to {}% of your quota", per)
            .as_str()
        )
    }
}
