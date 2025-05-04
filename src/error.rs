use std::path::PathBuf;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  Serde(#[from] serde_json::Error),
  #[error("Couldn't find an available \"{filename}\" from {}.", .current_dir.display())]
  NotFound {
    filename: String,
    current_dir: PathBuf,
  },
  #[error("Couldn't find an available {0} file")]
  MissingPackageJson(String),
}

