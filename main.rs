use std::os;
use std::io::fs;
use std::io;

fn possible_alternatives(project: String, environment: String, paths: Vec<Path>, command: &str) -> Vec<Path> {
  let mut alternatives_with_environment: Vec<Path> = Vec::new();
  let mut alternatives_without_environment: Vec<Path> = Vec::new();

  for path in paths.iter() {
    alternatives_with_environment.push(path.join(project + "-" + environment + "-" + command.into_string()));
    alternatives_without_environment.push(path.join(project + "-" + command.into_string()));
  }

  alternatives_with_environment + alternatives_without_environment
}

fn main() {
  let args = os::args();

  let project_prefix     = args[0].as_slice().split('/').last().unwrap().into_string();
  let environment_prefix = args.as_slice().get(1).map( |e| { e.clone() } ).unwrap_or("dev".into_string());
  let executable_paths   = os::getenv("PATH").map_or(Vec::new(), os::split_paths);

  println!("The project prefix is {}", project_prefix);
  println!("The environment is {}", environment_prefix);

  // This is very rudimentary, it doesn't check if a file can be executed by the
  // user, just that it can be executed by someone.
  fn is_executable(path: &Path) -> bool {
    let executable_flag = io::FilePermission::from_bits_truncate(
      io::USER_EXECUTE.bits() | io::GROUP_EXECUTE.bits() | io::OTHER_EXECUTE.bits()
    );

    fs::stat(path)
      .map( |stat| { stat.perm.intersects(executable_flag) } )
      .unwrap_or(false)
  }

  let executables: Vec<Path> = possible_alternatives(project_prefix, environment_prefix, executable_paths, "bar")
    .iter()
    .filter( |path: &&Path| { is_executable(path.clone()) } )
    .map( |p| p.clone() )
    .collect();

  for executable in executables.iter() {
    println!("{}", executable.display());
  }
}
