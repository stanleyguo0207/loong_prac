use sunface_course::kinds::PrimaryColor;
use sunface_course::utils::mix;

fn main() {
  let blue = PrimaryColor::Blue;
  let yellow = PrimaryColor::Yellow;
  println!("{:?}", mix(blue, yellow));
}
