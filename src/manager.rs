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
  /// Constructs a new, empty `PackageJsonManager`.
  pub fn new() -> Self {
    Default::default()
  }

  /// Constructs a new, empty `PackageJsonManger` with the specified `package.json` file path.
  /// ```
  /// use package_json::PackageJsonManager;
  /// let mut manager = PackageJsonManager::with_file_path("/path/to/package.json");
  ///
  pub fn with_file_path<FilePath>(path: FilePath) -> Self
  where
    FilePath: AsRef<Path>,
  {
    Self {
      file_path: Some(path.as_ref().to_path_buf()),
      json: PackageJson::default(),
    }
  }

  /// Try to locate the closest `package.json` file from [current working directory][std::env::current_dir] to sys root.
  pub fn locate_closest(&mut self) -> Option<PathBuf> {
    env::current_dir()
      .ok()
      .and_then(|cwd| self.locate_closest_from(cwd))
  }

  /// Try to locate the closest `package.json` file from specific directory to sys root.
  pub fn locate_closest_from<P: AsRef<Path>>(&mut self, from: P) -> Option<PathBuf> {
    fs::find_closest_file(PACKAGE_JSON_FILENAME, from).map(|file_path| {
      self.file_path = Some(file_path);
      self.file_path.as_ref().unwrap().to_owned()
    })
  }

  /// Specify the `package.json` file path which is used to read and write.
  pub fn set_file_path<FilePath: AsRef<Path>>(&mut self, file_path: FilePath) {
    self.file_path = Some(file_path.as_ref().to_path_buf());
  }

  /// Get the located file path after `locate_closest` or `locate_closest_from` evaluated.
  pub fn get_file_path(&mut self) -> Option<&Path> {
    self.file_path.as_deref()
  }

  /// Take the located file path out of `PackageJsonManager`, leaving a `None` in its place.
  pub fn take_file_path(&mut self) -> Option<PathBuf> {
    self.file_path.take()
  }

  /// Call file reader to read `package.json` file.
  fn read(&mut self) -> Result<()> {
    self
      .file_path
      .as_ref()
      .map(|file_path| {
        fs::read_json(file_path).map(|json| {
          self.json = json;
        })
      })
      .unwrap_or_else(|| {
        Err(format_err!(
          "Couldn't find an available {} file.",
          PACKAGE_JSON_FILENAME
        ))
      })
  }

  ///
  /// Evaluate `package.json` parser and return the immutable `PackageJson` reference.
  ///
  /// Note: It always reads the file again. In the most case, you should call `as_ref` to get a immutable reference if you have read it before.
  /// ```
  /// use package_json::PackageJsonManager;
  /// let mut manager = PackageJsonManager::new();
  /// if manager.locate_closest().is_some() {
  ///   assert!(manager.read_ref().is_ok());
  /// }
  /// ```
  pub fn read_ref(&mut self) -> Result<&PackageJson> {
    self.read().map(|_| &self.json)
  }

  /// Evaluate `package.json` parser and return the mutable `PackageJson` reference.
  ///
  /// Note: It always reads the file again. In the most case, you should call `as_mut` to get a mutable reference if you have read it before.
  /// ```
  /// use package_json::PackageJsonManager;
  /// let mut manager = PackageJsonManager::new();
  /// if manager.locate_closest().is_some() {
  ///   assert!(manager.read_mut().is_ok());
  /// }
  /// ```
  pub fn read_mut(&mut self) -> Result<&mut PackageJson> {
    self.read().map(|_| &mut self.json)
  }

  /// Use the current `package.json` content to write the target `package.json` file.
  /// ```
  /// use package_json::PackageJsonManager;
  /// let mut manager = PackageJsonManager::new();
  /// if manager.locate_closest().is_some() {
  ///   if let Ok(mut json) = manager.read_mut() {
  ///     json.name = "new name".to_string();
  ///     json.version = "1.0.0".to_string();
  ///   }
  ///   manager.write().expect("Couldn't write package.json");
  /// }
  /// ```
  pub fn write(&mut self) -> Result<()> {
    self
      .file_path
      .as_ref()
      .map(|file_path| fs::write_json(file_path, &self.json))
      .unwrap_or_else(|| {
        Err(format_err!(
          "Couldn't find an available {} file.",
          PACKAGE_JSON_FILENAME
        ))
      })
  }

  /// Write the current `package.json` content to the specific `package.json` file.
  /// ```
  /// use package_json::PackageJsonManager;
  /// use std::path::Path;
  /// let mut manager = PackageJsonManager::new();
  /// if manager.locate_closest().is_some() {
  ///   if let Ok(mut json) = manager.read_mut() {
  ///     json.name = "new name".to_string();
  ///     json.version = "1.0.0".to_string();
  ///   }
  ///   manager
  ///     .write_to(&Path::new("/path/to/package.json"))
  ///     .expect("Couldn't write package.json");
  /// }
  /// ```
  pub fn write_to(&mut self, file_path: &Path) -> Result<()> {
    fs::write_json(file_path, &self.json)
  }
}

impl AsRef<PackageJson> for PackageJsonManager {
  /// Return a immutable reference to the current `PackageJson` struct.
  fn as_ref(&self) -> &PackageJson {
    &self.json
  }
}

impl AsMut<PackageJson> for PackageJsonManager {
  /// Return a mutable reference to the current `PackageJson` struct.
  fn as_mut(&mut self) -> &mut PackageJson {
    &mut self.json
  }
}

#[test]
fn test_new() {
  use std::path::PathBuf;

  let mut manager = PackageJsonManager::new();
  assert_eq!(manager.file_path, None);

  let file_path = PathBuf::from("/path/to/package.json");
  manager.set_file_path(&file_path);
  assert_eq!(manager.file_path, Some(file_path.to_owned()));
  assert_eq!(manager.get_file_path(), Some(file_path.as_ref()));
  assert_eq!(manager.take_file_path(), Some(file_path.to_owned()));
  assert_eq!(manager.file_path, None);
}

#[test]
fn test_readable() {
  use crate::PACKAGE_JSON_FILENAME;
  use std::env::current_dir;
  use std::fs::{create_dir_all, File};
  use std::io::Write;
  use tempfile::tempdir_in;

  let mut manager = PackageJsonManager::new();
  debug_assert!(manager.read_ref().is_err(), "found an available file.");
  debug_assert!(
    manager.get_file_path().is_none(),
    "found an available file."
  );
  debug_assert!(
    manager.locate_closest().is_none(),
    "found an available file."
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
      debug_assert!(manager.read_ref().is_ok(), "read file failed.");
      debug_assert_eq!(manager.get_file_path().map(|p| p.to_path_buf()), expect);

      if let Ok(json) = manager.read_ref() {
        debug_assert_eq!(json.name, "test");
        debug_assert_eq!(json.version, "0.0.1");
      }

      let handler = manager.as_ref();
      debug_assert_eq!(handler.name, "test");
      debug_assert_eq!(handler.version, "0.0.1");
      debug_assert!(!handler.private);
    } else {
      debug_assert!(manager.read_ref().is_err(), "read field successful.")
    }
  }
}

#[test]
fn test_writable() {
  use crate::PACKAGE_JSON_FILENAME;
  use std::env::current_dir;
  use std::fs::{create_dir_all, File};
  use std::io::Write;
  use tempfile::tempdir_in;

  let mut manager = PackageJsonManager::new();
  debug_assert!(manager.write().is_err(), "found an available file.");

  let dir = tempdir_in(current_dir().unwrap()).expect("create temp_dir failed!");
  let file_path = dir.path().join(format!("a/b/c/{}", PACKAGE_JSON_FILENAME));
  let file_dir = file_path.parent().unwrap();
  create_dir_all(file_dir).expect("create a/b/c dir failed!");

  let mut valid_file = File::create(&file_path).expect("create file failed!");
  let valid_json = r#"{
"name": "test",
"version": "0.0.1"
}"#;

  valid_file
    .write_all(valid_json.as_bytes())
    .expect("write json failed");

  manager.set_file_path(&file_path);

  // case `read_mut`
  {
    let file_writer = manager.read_mut();
    debug_assert!(
      file_writer.is_ok(),
      "{}",
      format!("create file writer failed: {:?}", file_writer)
    );
    if let Ok(mut json) = file_writer {
      json.name = "test2".to_string();
      json.version = "0.0.2".to_string();
      debug_assert!(manager.write().is_ok());
    }
    let file_reader = manager.as_ref();
    debug_assert_eq!(file_reader.name, "test2");
    debug_assert_eq!(file_reader.version, "0.0.2");
  }

  // case `as_mut`
  {
    let mut mutable_handler = manager.as_mut();
    mutable_handler.name = "test3".to_string();
    mutable_handler.version = "0.0.3".to_string();
    let file_reader = manager.as_ref();
    debug_assert_eq!(file_reader.name, "test3");
    debug_assert_eq!(file_reader.version, "0.0.3");
  }
}
