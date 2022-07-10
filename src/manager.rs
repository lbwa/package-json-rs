use crate::fs;
use crate::PackageJson;
use anyhow::{format_err, Result};
use std::env;
use std::path::{Path, PathBuf};

pub const PACKAGE_JSON_FILENAME: &str = "package.json";

/// A manager for manipulating `package.json` file.
#[derive(Debug, Default)]
pub struct PackageJsonManager {
  file_path: Option<PathBuf>,
  json: PackageJson,
}

impl PackageJsonManager {
  /// create a `PackageJsonManager` instance.
  pub fn new() -> Self {
    Default::default()
  }

  /// locate the closest `package.json` file from [current working directory][std::env::current_dir].
  pub fn locate_closest(&mut self) -> Option<PathBuf> {
    env::current_dir()
      .ok()
      .and_then(|cwd| self.locate_closest_from(cwd))
  }

  /// locate the closest `package.json` file from specific directory.
  pub fn locate_closest_from<P: AsRef<Path>>(&mut self, from: P) -> Option<PathBuf> {
    fs::find_closest_file(PACKAGE_JSON_FILENAME, from).map(|file_path| {
      self.json = PackageJson::default();
      self.file_path = Some(file_path);
      self.file_path.as_ref().unwrap().to_owned()
    })
  }

  /// parse `package.json` from [`locate_closest()`][PackageJsonManager::locate_closest] or [`locate_closest_from(env.current_dir()?)`][PackageJsonManager::locate_closest_from]
  pub fn read(&mut self) -> Result<&PackageJson> {
    if let Some(ref file_path) = self.file_path {
      fs::read_json::<PackageJson, &Path>(file_path).map(|json| {
        self.json = json;
        &self.json
      })
    } else {
      Err(format_err!(format!(
        "Couldn't find available {} file.",
        PACKAGE_JSON_FILENAME
      )))
    }
  }

  /// get located file path
  pub fn get_file_path(&mut self) -> Option<&Path> {
    self.file_path.as_deref()
  }

  /// take located file path
  pub fn take_file_path(&mut self) -> Option<PathBuf> {
    self.file_path.take()
  }
}

impl AsRef<PackageJson> for PackageJsonManager {
  fn as_ref(&self) -> &PackageJson {
    &self.json
  }
}

#[test]
fn test_package_json_manager() {
  let manager = PackageJsonManager::new();
  assert_eq!(manager.file_path, None);
}

#[test]
fn test_package_json_manager_read() {
  use crate::PACKAGE_JSON_FILENAME;
  use std::env::current_dir;
  use std::fs::{create_dir_all, File};
  use std::io::Write;
  use tempfile::tempdir_in;

  let mut manager = PackageJsonManager::new();
  debug_assert!(manager.read().is_err(), "found a available file.");
  debug_assert!(manager.get_file_path().is_none(), "found a available file.");
  debug_assert!(
    manager.locate_closest().is_none(),
    "found a available file."
  );

  let dir = tempdir_in(current_dir().unwrap()).expect("create temp_dir failed!");
  let file_path = dir.path().join(format!("a/b/c/{}", PACKAGE_JSON_FILENAME));
  let file_dir = file_path.parent().unwrap();
  let deeper_file_dir = file_dir.join("d/e/f");
  create_dir_all(file_dir).expect("create a/b/c dir failed!");
  create_dir_all(deeper_file_dir.as_path()).expect("create d/e/f dir field!");

  let mut valid_file = File::create(&file_path).expect("create file failed!");
  let valid_json = r#"{
  "name": "test",
  "version": "0.0.1"
}"#;

  valid_file
    .write_all(valid_json.as_bytes())
    .expect("write json failed");

  for (dir, expect) in [
    (dir.path(), None),
    (file_dir, Some(file_path.to_owned())),
    (deeper_file_dir.as_path(), Some(file_path.to_owned())),
  ] {
    debug_assert_eq!(manager.locate_closest_from(dir), expect);
    if expect.is_some() {
      debug_assert!(manager.read().is_ok(), "read file failed.")
    } else {
      debug_assert!(manager.read().is_err(), "read field successful.")
    }
  }
}
