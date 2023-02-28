use std::ffi::OsString;
use std::path::{PathBuf};
use std::env;

pub fn home()-> Option<PathBuf> {
  let home = match env::var_os("HOME")  {
    Some(c) => c,
    None => OsString::new()
  };
  let home_str = home.to_str().unwrap();
  let path = PathBuf::from(home_str);

  if path.is_dir() {
    return Some(path)
  }
  return None
}

pub fn dot_config()-> Option<PathBuf> {
  let xdg_config_home = match env::var_os("XDG_CONFIG_HOME") {
    Some(c) => c,
    None => OsString::new()
  };
  let xdg_config_home_str = xdg_config_home.to_str().unwrap();
  let home_dir = match home() {
    Some(home) => home,
    None => PathBuf::new()
  };
  let fallback_dir = home_dir.join(".config");

  let mut path_str: String = if !xdg_config_home_str.is_empty() { String::from(xdg_config_home_str) } else { fallback_dir.to_str().unwrap().to_string() };
  path_str.push_str("/rofi-totp/");
  let path = PathBuf::from(path_str);

  if path.is_dir() {
    return Some(path)
  }
  return None
}