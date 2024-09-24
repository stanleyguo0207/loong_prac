use std::fmt;

struct WrapperVec(Vec<String>);

impl fmt::Display for WrapperVec {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn main() {
  let wv = WrapperVec(vec![String::from("hello"), String::from("world")]);
  println!("{}", wv);
}
