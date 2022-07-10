use anyhow::{format_err, Result};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn find_closest_file(filename: &str, current_dir: PathBuf) -> Option<PathBuf> {
  let mut current_dir = current_dir;
  loop {
    let file_path = current_dir.join(filename);
    if file_path.exists() {
      return Some(file_path);
    }
    if !current_dir.pop() {
      return None;
    }
  }
}

pub fn read_json<Json>(filepath: &PathBuf) -> Result<Json>
where
  Json: serde::de::DeserializeOwned,
{
  let mut file = File::open(filepath)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let serialized_json = serde_json::from_str(&contents);

  match serialized_json {
    Ok(json) => Ok(json),
    Err(error) => Err(format_err!(error)),
  }
}

#[test]
fn test_find_closest_file() {
  use std::env::current_dir;
  use std::fs::create_dir_all;
  use std::io::Write;
  use tempfile::tempdir_in;

  let dir = tempdir_in(current_dir().unwrap()).expect("create temp_dir failed!");
  let file_path = dir.path().join("nest/a/b/c/package.json");
  let deeper_dir = file_path.parent().unwrap().join("d/e/f");
  create_dir_all(file_path.parent().unwrap()).expect("create package.json dir failed!");
  create_dir_all(&deeper_dir).expect("create nest/a/b/c/d/e/f dir failed!");

  let mut valid_file = File::create(&file_path).expect("create file failed!");
  let valid_json = r#"{
  "name": "test"
}
  "#;

  valid_file
    .write_all(valid_json.as_bytes())
    .expect("write json failed");

  for (dir, expect) in [
    // the parent dir
    (dir.path().to_path_buf(), None),
    // the sibling of parent dir
    (
      file_path.parent().unwrap().parent().unwrap().join("ff"),
      None,
    ),
    // the file dir
    (
      file_path.parent().unwrap().to_path_buf(),
      Some(file_path.to_owned()),
    ),
    // the file path
    (file_path.to_path_buf(), Some(file_path.to_owned())),
    // the children dir of file_path
    (deeper_dir.to_path_buf(), Some(file_path.to_owned())),
  ] {
    debug_assert_eq!(
      find_closest_file("package.json", dir),
      expect,
      "find_closest_file failed!"
    );
  }
}
