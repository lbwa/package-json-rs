//!
//! Use the `package-json` crate to manage your `package.json` file.
//!
//! ## How to locate the closest `package.json` file
//!
//! ```ignore
//! use package_json::PackageJsonManager;
//! use std::path::Path;
//! let mut manager = PackageJsonManager::new();
//! // based on the current working directory
//! assert!(manager.locate_closest().is_some());
//! // based on the given path
//! assert!(manager.locate_closest_from(&Path::new("/path/to/dir")).is_some());
//! ```
//!
//! Use [`with_file_path`][PackageJsonManager::with_file_path] to create a [`PackageJsonManager`][PackageJsonManager] instance with the give file path.
//!
//! ```
//! use std::path::Path;
//! use package_json::PackageJsonManager;
//! let mut manager = PackageJsonManager::with_file_path(&Path::new("/path/to/package.json"));
//! ```
//!
//! Use [`set_file_path`][PackageJsonManager::set_file_path] to change file path.
//!
//! ## How to read or write the current `package.json` file
//!
//! We can use [`read_mut`][PackageJsonManager::read_mut] or [`read_ref`][PackageJsonManager::read_ref] to read the current `package.json` file after file located.
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
//! On the other hand, we should use [`as_mut`][PackageJsonManager::as_mut] and [`as_ref`][PackageJsonManager::as_ref] to get a mutable reference or a immutable reference if we have read it before.
//! ```
//! use package_json::PackageJsonManager;
//! let mut manager = PackageJsonManager::new();
//! if manager.locate_closest().is_some() {
//!   // manager.as_mut();
//!   // manager.as_ref();
//! }
//! ```
//! Use [`write`][PackageJsonManager::write] to write the current `package.json` file to the disk. If we want to change the output path, eg. create a new `package.json`, we should use [`write_to`][PackageJsonManager::write_to] instead.
//! ```ignore
//! assert!(manager.write().is_ok());
//! ```
//!

mod fs;
mod manager;
mod schema;

pub use crate::manager::{PackageJsonManager, PACKAGE_JSON_FILENAME};
pub use crate::schema::*;
