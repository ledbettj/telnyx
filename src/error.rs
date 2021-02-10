#[derive(Debug)]
pub enum Error {
  HttpError(hyper::http::Error),
  InternalError(hyper::Error),
  ParseError(serde_json::Error)
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::HttpError(e) => Some(e),
      Error::ParseError(e) => Some(e),
      Error::InternalError(e) => Some(e),
      _ => None
    }
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    self.source()
  }
}

impl From<hyper::http::Error> for Error {
  fn from(e: hyper::http::Error) -> Self {
    Error::HttpError(e)
  }
}


impl From<hyper::Error> for Error {
  fn from(e: hyper::Error) -> Self {
    Error::InternalError(e)
  }
}


impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Error::ParseError(e)
  }
}
