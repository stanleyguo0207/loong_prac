use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

fn main() {
  Sunfei::hello_macro();
  Sunface::hello_macro();
}
