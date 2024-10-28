
#[derive(Debug, PartialEq, Eq)]

pub struct Message {
content:  String,
user:  String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
          Message{content: ms, user: u}
  }
  pub fn send_ms(&self) -> Option<&str> {
          if self.content.is_empty() || self.content.contains("stupid") {
            return None;
          }
          Some(&self.content)
  }
}


pub fn check_ms(ms: &Message) -> (bool, &str) {
          if ms.send_ms() == None {
            return  (false, "ERROR: illegal");
          }
          (true, "hello there")
}