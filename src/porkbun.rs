use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Auth {
    apikey: String,
    secretapikey: String,
}

pub struct Porkbun {
    base_url: String,
    client: Client,
    domain: String,
    auth: Auth,
}

impl Porkbun {
    pub fn new(api_key: String, secret_key: String, domain: String) -> Self {
        let client = Client::builder().build().unwrap();
        let auth = Auth {
            apikey: api_key,
            secretapikey: secret_key,
        };
        Porkbun {
            base_url: String::from("https://porkbun.com/api/json/v3"),
            client,
            domain,
            auth,
        }
    }

    pub async fn get_record(
        &self,
        subdomain: &str,
        record_type: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/dns/retrieveByNameType/{}/{}/{}",
            self.base_url, self.domain, record_type, subdomain
        );
        let auth = serde_json::to_string(&self.auth)?;
        let response = self.client.post(url).body(auth).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}
