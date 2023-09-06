use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use home;

pub const FILE_NAME: &'static str = ".happy_todo";

pub fn init() {
  let file_path = home::home_dir().unwrap().join(PathBuf::from(FILE_NAME));
  if Path::new(&file_path).exists() {
    println!("存储文件已存在，不再进行初始化！");
    return;
  }

  File::create(&file_path).expect("创建文件失败");

  println!("已成功初始化存储文件")
}

pub fn get_content() -> String {
  let file_path = home::home_dir().unwrap().join(PathBuf::from(FILE_NAME));
  fs::read_to_string(&file_path).unwrap()
}

pub fn set_content(s: String) -> io::Result<()> {
  println!("写入内容：{}", &s);
  let file_path = home::home_dir().unwrap().join(PathBuf::from(FILE_NAME));
  let mut file = OpenOptions::new().append(true).open(&file_path)?;
  file.write(s.as_bytes())?;

  Ok(())
}