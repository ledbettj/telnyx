use std::default::Default;

pub struct Credentials {
  pub(crate) api_base: String,
  pub(crate) api_key: String,
}

impl Default for Credentials {
  fn default() -> Self {
    Self {
      api_base: "https://api.telnyx.com/v2".into(),
      api_key: "".into(),
    }
  }
}

impl Credentials {
  pub fn new<S: Into<String>>(api_key: S) -> Self {
    Self {
      api_key: api_key.into(),
      ..Default::default()
    }
  }

  pub fn custom<S: Into<String>>(api_key: S, base: S) -> Self {
    Self {
      api_key: api_key.into(),
      api_base: base.into(),
    }
  }
}
