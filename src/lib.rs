//!
//! Use the `package-json` crate to manage your `package.json` file.
//!
//! ## How to locate the closest `package.json` file
//!
//! ```rust
//! use package_json::PackageJsonManager;
//! let mut manager = PackageJsonManager::new();
//! assert!(manager.locate_closest().is_some()); // based on the current working directory
//! assert!(manager.locate_closest_from(&Path::new("/path/to/dir")).is_some()); // based on the given path
//! ```
//!
//! We also provide an associated function `with_file_path` to specify the closest `package.json` file from the given path.
//!
//! ```
//! use package_json::PackageJsonManager;
//! let mut manager = PackageJsonManager::with_file_path(&Path::new("/path/to/package.json"));
//! ```
//!
//! ## How to read or write the current `package.json` file
//!
//! You can call `read_mut` or `read_ref` to read the current `package.json` file.
//! They always invoke file reader to parse `package.json` file to PackageJson struct.
//!
//! ```
//! use package_json::PackageJsonManager;
//! let mut manager = PackageJsonManager::new();
//! if manager.locate_closest().is_some() {
//!   assert!(manager.read_mut().is_ok());
//!   assert!(manager.read_ref().is_ok());
//! }
//! ```
//!
//! On the other hand, we also provide `as_mut` and `as_ref` to get a mutable reference or a immutable reference if you have read it before.
//! ```
//! use package_json::PackageJsonManager;
//! let mut manager = PackageJsonManager::new();
//! if manager.locate_closest().is_some() {
//!   assert!(manager.as_mut().is_ok());
//!   assert!(manager.as_ref().is_ok());
//! }
//! ```
//!

mod fs;
mod manager;
mod schema;

pub use crate::manager::{PackageJsonManager, PACKAGE_JSON_FILENAME};
pub use crate::schema::*;
