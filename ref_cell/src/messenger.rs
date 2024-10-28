pub use std::cell::RefCell;
pub use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
   pub logger: &'a dyn Logger,
   pub value: usize,
   pub max: usize,
}

impl <'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Self {
            logger: logger,
            value: 0,
            max: max,
        }
    }


    pub fn set_value(&self, value: &Rc<usize>) {
    let val = Rc::strong_count(value) ;
        let percentage = ( val * 100)/ self.max ;
        // println!("{}", val);
        
        if percentage >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70 && percentage < 100  {
            self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", percentage));
        }
        // *self.value.borrow_mut() += 1;

    }

    pub fn peek(&self,value: &Rc<usize>) {  
        let val =  Rc::strong_count(value);
        let percentage = (val  * 100) / self.max  ;
        self.logger.info(&format!("Info: you are using up to {}% of your quota", percentage));
    }
}
