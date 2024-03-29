/// Indicates if the snippet should be HTML encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Encoder {
  /// No encoding
  Default,

  /// HTML-escape the snippet text and then insert the highlighting tags
  Html,
}
