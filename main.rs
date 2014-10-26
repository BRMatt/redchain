use std::os;

fn get_path() -> Option<String> {
  return os::getenv("PATH");
}

fn split_path<'a>(path: String) -> Vec<String> {
  path.as_slice().split(':').map(|x| { x.into_string() }).collect()
}

fn main() {
  let args = os::args();

  let project_prefix     = args[0].as_slice().split('/').last().unwrap().into_string();
  let environment_prefix = args.as_slice().get(1).map( |e| { e.clone() } ).unwrap_or("dev".into_string());

  println!("The project prefix is {}", project_prefix);
  println!("The environment is {}", environment_prefix);

  match get_path().map(split_path) {
    Some(val) => for p in val.iter() { println!("{}", p); },
    None => println!("Where is the path?!")
  }
}
