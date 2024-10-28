#[derive(Debug,PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
          if first == self.r  && second == self.g{
             Color{
                r : second,
                g : first,
                b : self.b,
                a : self.a,
             }
          }else  if first == self.r  && second == self.b{
            Color{
                r : second,
                g : self.g,
                b : first,
                a : self.a,
             }
          }else  if first == self.r  && second == self.a{
            Color{
                r : second,
                g : self.g,
                b : self.b,
                a : first,
             }
          } else if first == self.g  && second == self.r{
            Color{
               r : first,
               g : second,
               b : self.b,
               a : self.a,
            }
         }else  if first == self.g  && second == self.b{
           Color{
               r : self.r,
               g : second,
               b : first,
               a : self.a,
            }
         }else  if first == self.g  && second == self.a{
           Color{
               r : self.r,
               g : second,
               b : self.b,
               a : first,
            }
         }else if first == self.b  && second == self.r{
            Color{
               r : first,
               g : self.g,
               b : second,
               a : self.a,
            }
         }else  if first == self.b  && second == self.g{
           Color{
               r : self.r,
               g : first,
               b : second,
               a : self.a,
            }
         }else  if first == self.b  && second == self.a{
           Color{
               r : self.r,
               g : self.g,
               b : second,
               a : first,
            }
         }else if first == self.a  && second == self.r{
            Color{
               r : first,
               g : self.g,
               b : self.b,
               a : second,
            }
         }else  if first == self.a  && second == self.g{
           Color{
               r : self.r,
               g : first,
               b : self.b,
               a : second,
            }
         }else {
           Color{
               r : self.r,
               g : self.g,
               b : first,
               a : second,
            }
         }
    }
}
