use std::fs::File;
use std::io::{Read};
use yaml_rust::{YamlLoader};

use totp;
use dir;

pub fn get_list() -> Result<Vec<std::string::String>, String> {

  let mut final_otps : Vec<std::string::String> = Vec::new();
  
  let config_dir = match dir::dot_config() {
    Some(home) => home,
    None => {
      return Err(String::from("Can't find the default config directory"));
    }
  };

  let config_path = config_dir.join("2fa.yml");

  let mut config_file = match File::open(&config_path) {
    Err(_) => {
      return Err(format!("Make sure you have created the following config file {}", config_path.to_str().unwrap().to_string()));
    }
    Ok(file) => file,
  };

  let mut s = String::new();
  let content = match config_file.read_to_string(&mut s) {
    Err(_) => {
      return Err(String::from("Make sure your config file is readable"));
    }Ok(_) => s,
  };

  let config = YamlLoader::load_from_str(&content).unwrap();

  let apps = &config[0]["apps"].as_vec().unwrap();
  
  for app in apps.iter() {
    let label = app["label"].as_str().unwrap();
    let secret = app["secret"].as_str().unwrap();
    let otp = totp::generate(secret).unwrap();
    final_otps.push(vec![label, &otp.to_string()].join(" :: "))
  }

  Ok(final_otps)

}