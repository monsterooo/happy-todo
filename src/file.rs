use std::path::{Path, PathBuf};
use std::fs::{self, File};
use home;

pub const FILE_NAME: &'static str = ".happy_todo";

pub fn init() {
  let home_dir = home::home_dir().unwrap().join(PathBuf::from(FILE_NAME));

  if Path::new(&home_dir).exists() {
    println!("存储文件已存在，不再进行初始化！");
    return;
  }

  File::create(home_dir).expect("创建文件失败");

  println!("已成功初始化存储文件")
}

pub fn get_content() -> String {
  let home_dir = home::home_dir().unwrap().join(PathBuf::from(FILE_NAME));
  fs::read_to_string(home_dir).unwrap()
}