use std::os;

fn main() {
  match os::getenv("PATH") {
    Some(val) => println!("This is your path: {}", val),
    None      => println!("You don't have a PATH set in your environment")
  }

}
