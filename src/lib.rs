use hyper;
use hyper_tls;

pub mod resources;
pub mod credentials;
pub mod error;

pub type Credentials = credentials::Credentials;
pub type Error = error::Error;

type HyperClient =
  hyper::Client<hyper_tls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::Body>;

pub(crate) fn client() -> HyperClient {
  hyper::Client::builder().build(hyper_tls::HttpsConnector::new())
}

#[cfg(test)]
mod tests {
  use crate::*;

  fn credentials() -> Credentials {
    Credentials::custom("KEY0123456", "http://localhost:12111/v2")
  }

  #[tokio::test]
  async fn test_list_number_orders() {
    let c = credentials();
    let r = resources::NumberOrder::list(&c).await.unwrap();

    println!("rc = {:?}", r);
  }

  #[tokio::test]
  async fn test_list_messages() {
    let c = credentials();
    let r = resources::Message::get(&c, "abcd").await.unwrap();

    println!("rc = {:?}", r);
  }
}
