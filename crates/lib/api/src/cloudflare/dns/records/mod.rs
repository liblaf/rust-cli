use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cloudflare::Cloudflare;

use super::Dns;

pub struct Records {
    cloudflare: Cloudflare,
}

impl Dns {
    pub fn records(&self) -> Records {
        Records {
            cloudflare: self.cloudflare.clone(),
        }
    }
}

impl Records {
    pub async fn create(
        &self,
        zone_id: &str,
        params: &RecordCreateParams,
    ) -> anyhow::Result<Record> {
        let request = self
            .cloudflare
            .post(format!("/zones/{}/dns_records", zone_id))
            .json(params);
        let response = self.cloudflare.send::<Record>(request).await?;
        Ok(response)
    }

    pub async fn list(&self, zone_id: &str) -> anyhow::Result<Vec<Record>> {
        let request = self
            .cloudflare
            .get(format!("/zones/{}/dns_records", zone_id));
        let response = self.cloudflare.send::<Vec<Record>>(request).await?;
        Ok(response)
    }

    pub async fn delete(&self, dns_record_id: &str, zone_id: &str) -> anyhow::Result<()> {
        let request = self
            .cloudflare
            .delete(format!("/zones/{}/dns_records/{}", zone_id, dns_record_id));
        self.cloudflare.send::<Value>(request).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RecordCreateParams {
    pub content: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Record {
    pub content: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub proxied: bool,
    pub ttl: u64,
}
