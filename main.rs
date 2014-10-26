use std::os;

fn get_path() -> Option<String> {
  return os::getenv("PATH");
}

fn split_path<'a>(path: String) -> Vec<String> {
  path.as_slice().split(':').map(|x| { x.into_string() }).collect()
}

fn main() {
  match get_path().map(split_path) {
    Some(val) => for p in val.iter() { println!("{}", p); },
    None => println!("Where is the path?!")
  }
}
