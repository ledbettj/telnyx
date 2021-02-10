use serde::Deserialize;

trait ApiResource {
  fn index_endpoint(prefix: &str) -> String;
  fn item_endpoint(prefix: &str, id: &str) -> String;
}

macro_rules! telnyx_resource {
  ($type:ty, $resource_type:path, $url_fragment:literal) => {
    impl ApiResource for $type {
      fn index_endpoint(prefix: &str) -> String {
        format!("{}/{}", prefix, $url_fragment).into()
      }
      fn item_endpoint(prefix: &str, id: &str) -> String {
        format!("{}/{}/{}", prefix, $url_fragment, id).into()
      }
    }
    impl $type {
      pub async fn list(creds: &crate::Credentials) -> Result<Vec<Self>, crate::Error> {
        let req = hyper::Request::builder()
          .uri(Self::index_endpoint(&creds.api_base))
          .header("Authorization", format!("Bearer {}", creds.api_key))
          .body(hyper::Body::empty())?;

        let result = crate::client().request(req).await?;

        let body = hyper::body::to_bytes(result.into_body()).await?;
        let wrapper : ListResponseWrapper = serde_json::from_slice(&body)?;

        let items = wrapper
          .data
          .iter()
          .filter_map(|resource| {
            if let $resource_type(t) = resource {
              Some(t)
            } else {
              None
            }
          })
          .cloned()
          .collect();

        Ok(items)
      }

      pub async fn get<S: AsRef<str>>(creds: &crate::Credentials, id: S) -> Result<Self, crate::Error> {
        let req = hyper::Request::builder()
          .uri(Self::item_endpoint(&creds.api_base, id.as_ref()))
          .header("Authorization", format!("Bearer {}", creds.api_key))
          .body(hyper::Body::empty())?;

        let result = crate::client().request(req).await?;

        let body = hyper::body::to_bytes(result.into_body()).await?;
        let wrapper : ItemResponseWrapper = serde_json::from_slice(&body)?;

        if let $resource_type(t) = wrapper.data {
          Ok(t)
        } else {
          panic!("shit")
        }
      }
    }
  }
}

#[derive(Deserialize)]
pub struct ListResponseWrapper {
  pub data: Vec<Resource>
}

#[derive(Deserialize)]
pub struct ItemResponseWrapper {
  pub data: Resource
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "record_type")]
pub enum Resource {
  #[serde(alias="number_order")]
  NumberOrder(NumberOrder),
  #[serde(alias="message")]
  Message(Message),
}

#[derive(Deserialize, Debug, Clone)]
pub struct NumberOrder {
  pub connection_id: Option<String>,
  pub billing_group_id: Option<String>,
  pub created_at: String,
  pub customer_reference: Option<String>,
  pub id: String,
  pub message_profile_id: Option<String>,
  pub phone_numbers_count: usize,
  pub requirements_met: bool,
  pub status: String,
  pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
  pub id: String,
  pub direction: String,
  pub r#type: String,
  pub text: String,
  pub webhook_url: String,
  pub webhook_failover_url: String,
  pub use_profile_webhooks: Option<bool>,
  pub parts: usize,
  pub created_at: String,
  pub updated_at: String,
  pub valid_until: Option<String>,
  pub carrier: String,
  pub line_type: String,
}

telnyx_resource!(NumberOrder, Resource::NumberOrder, "number_orders");
telnyx_resource!(Message, Resource::Message, "messages");
