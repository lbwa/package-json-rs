use derive_builder::Builder;

/// Options used for writing a file.
#[derive(Debug, Builder, Default)]
pub struct WriteOptions {
  /// Set this to `false` to generate a compact JSON, instead of a pretty-printed JSON.
  #[builder(default = "true")]
  pub pretty: bool,
}
