use std::collections::HashMap;
pub mod messenger;
pub use  messenger::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Worker{
   pub track_value :  Rc<usize>,
   pub mapped_messages : RefCell<HashMap<String, String>>,
   pub all_messages : RefCell<Vec<String>>,
}


impl Worker {
    pub fn new(value: usize)-> Self{
          Self{
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
            }
    }
}

impl Logger for Worker{
  fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg[9..].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
        // println!("{}$$$$", msg);
    }

  fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg[6..].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }

    
  fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg[7..].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}