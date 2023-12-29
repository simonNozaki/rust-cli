use std::io::Error;
use rust_cli::utils::reader::Program;
use rust_cli::utils::yarn::{extract_scripts, get_package_json_str};

fn main() -> Result<(), Error> {
  let dir = ""; // 使うときにpathを指定する
  let buffer = get_package_json_str(&dir)?;

  let scripts = extract_scripts(&buffer);
  for (k, v) in scripts {
    println!("{} => {}", &k, &v);
  }

  let _ = Program { command: "ls".to_string(), args: vec![], dir: dir.to_string() }.execute();

  let yarn = Program { command: "yarn".to_string(), args: vec!["".to_string()], dir: dir.to_owned() }.execute_async();
  if let Ok(status) = yarn {
    println!("Exit status: {}", status);
  }

  Ok(())
}
