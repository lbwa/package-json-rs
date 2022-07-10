//!
//! Use the `package-json` crate to manage your `package.json` file.
//!

mod fs;
mod schema;

pub use crate::manager::PackageJsonManager;
pub use crate::schema::*;
