use std::collections::HashMap;
use std::num::ParseFloatError;


#[derive(Debug,PartialEq, Eq)]
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", l_h.chars().next().unwrap()),
            long_hand: format!("--{}", l_h),
            desc: d.to_string(),
        }
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

pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) ->String{
     match self.flags.get(&flag) {
        // println!("{:?}",Ok(result));
        Some(func) => match func(argv[0], argv[1]) {
            Ok(result) => result ,
            Err(err) => err.to_string(),
        },
        None => "Flag not found".to_string()
    }
}

    
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f32 = a.parse()?;
    let y: f32 = b.parse()?;
    Ok((x / y).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f32 = a.parse()?;
    let y: f32 = b.parse()?;
    Ok((x % y).to_string())
}



