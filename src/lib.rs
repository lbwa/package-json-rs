//!
//! Use the `package-json` crate to manage your `package.json` file.
//!

mod fs;
mod manager;
mod schema;

pub use crate::manager::{PackageJsonManager, PACKAGE_JSON_FILENAME};
pub use crate::schema::*;
