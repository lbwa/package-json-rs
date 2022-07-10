use anyhow::{format_err, Result};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn find_closest_file<P: AsRef<Path>>(filename: &str, current_dir: P) -> Option<PathBuf> {
  let mut current_dir = PathBuf::from(current_dir.as_ref());
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

pub fn read_json<Json, FilePath>(file_path: FilePath) -> Result<Json>
where
  Json: serde::de::DeserializeOwned,
  FilePath: AsRef<Path>,
{
  let mut file = File::open(file_path)?;
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
  use crate::PACKAGE_JSON_FILENAME;
  use std::env::current_dir;
  use std::fs::create_dir_all;
  use std::io::Write;
  use tempfile::tempdir_in;

  let dir = tempdir_in(current_dir().unwrap()).expect("create temp_dir failed!");
  let file_path = dir
    .path()
    .join(format!("nest/a/b/c/{}", PACKAGE_JSON_FILENAME));
  let deeper_dir = file_path.parent().unwrap().join("d/e/f");
  create_dir_all(file_path.parent().unwrap())
    .unwrap_or_else(|_| panic!("create {} dir failed!", PACKAGE_JSON_FILENAME));
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
    (deeper_dir, Some(file_path.to_owned())),
  ] {
    debug_assert_eq!(
      find_closest_file(PACKAGE_JSON_FILENAME, dir),
      expect,
      "find_closest_file failed!"
    );
  }
}
