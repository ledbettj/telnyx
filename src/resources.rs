use log;
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

        log::debug!("{:?}", req);
        let result = crate::client().request(req).await?;
        log::debug!("{:?}", result);
        let body = hyper::body::to_bytes(result.into_body()).await?;
        log::debug!("Body = {:?}", body);
        let wrapper: ListResponseWrapper = serde_json::from_slice(&body)?;

        let items: Result<Vec<Self>, crate::Error> = wrapper
          .data
          .iter()
          .map(|resource| {
            if let $resource_type(t) = resource {
              Ok(t.clone())
            } else {
              log::error!(
                "Expected a resource of type {}, got {:?}",
                stringify!($resource_type),
                resource
              );
              Err(crate::Error::ResourceMismatchError)
            }
          })
          .collect();

        items
      }

      pub async fn get<S: AsRef<str>>(
        creds: &crate::Credentials,
        id: S,
      ) -> Result<Self, crate::Error> {
        let req = hyper::Request::builder()
          .uri(Self::item_endpoint(&creds.api_base, id.as_ref()))
          .header("Authorization", format!("Bearer {}", creds.api_key))
          .body(hyper::Body::empty())?;

        log::debug!("{:?}", req);
        let result = crate::client().request(req).await?;
        log::debug!("{:?}", result);
        let body = hyper::body::to_bytes(result.into_body()).await?;
        log::debug!("Body = {:?}", body);
        let wrapper: ItemResponseWrapper = serde_json::from_slice(&body)?;

        if let $resource_type(t) = wrapper.data {
          Ok(t)
        } else {
          log::error!(
            "Expected a resource of type {}, got {:?}",
            stringify!($resource_type),
            wrapper.data
          );
          Err(crate::Error::ResourceMismatchError)
        }
      }
    }
  };
}

#[derive(Deserialize)]
pub struct ListResponseWrapper {
  pub data: Vec<Resource>,
}

#[derive(Deserialize)]
pub struct ItemResponseWrapper {
  pub data: Resource,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "record_type")]
pub enum Resource {
  #[serde(alias = "number_order")]
  NumberOrder(NumberOrder),
  #[serde(alias = "message")]
  Message(Message),
  #[serde(alias = "available_phone_number")]
  AvailablePhoneNumber(AvailablePhoneNumber),
}

#[derive(Deserialize, Debug, Clone)]
pub struct NumberOrder {
  pub billing_group_id: Option<String>,
  pub connection_id: Option<String>,
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
pub struct MessageAddress {
  pub carrier: String,
  pub line_type: String,
  pub phone_number: String,
  pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageMedia {
  pub content_type: Option<String>,
  pub sha256: Option<String>,
  pub size: Option<usize>,
  pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageCost {
  pub amount: String,
  pub currency: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
  pub completed_at: Option<String>,
  pub cost: Option<MessageCost>,
  pub direction: String,
  pub encoding: String,
  pub from: MessageAddress,
  pub id: String,
  #[serde(default = "Vec::new")]
  pub media: Vec<MessageMedia>,
  pub messaging_profile_id: String,
  pub parts: usize,
  pub received_at: String,
  pub sent_at: Option<String>,
  pub subject: Option<String>,
  #[serde(default = "Vec::new")]
  pub tags: Vec<String>,
  pub text: String,
  pub to: Vec<MessageAddress>,
  pub r#type: String,
  pub valid_until: Option<String>,
  pub webhook_failover_url: String,
  pub webhook_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AvailablePhoneNumberCost {
  pub currency: String,
  pub monthly_cost: String,
  pub upfront_cost: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AvailablePhoneNumberRegion {
  pub region_name: String,
  pub region_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AvailablePhoneNumberRegulatoryRequirements {
  pub description: String,
  pub field_type: String,
  pub label: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AvailablePhoneNumber {
  pub best_effort: bool,
  pub cost_information: AvailablePhoneNumberCost,
  pub phone_number: String,
  pub quickship: bool,
  #[serde(default = "Vec::new")]
  pub region_information: Vec<AvailablePhoneNumberRegion>,
  #[serde(default = "Vec::new")]
  pub regulatory_requirements: Vec<AvailablePhoneNumberRegulatoryRequirements>,
  pub reservable: bool,
  pub vanity_format: String,
}

telnyx_resource!(NumberOrder, Resource::NumberOrder, "number_orders");
telnyx_resource!(Message, Resource::Message, "messages");
telnyx_resource!(
  AvailablePhoneNumber,
  Resource::AvailablePhoneNumber,
  "available_phone_numbers"
);
