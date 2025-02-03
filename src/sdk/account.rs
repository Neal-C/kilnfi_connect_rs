use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::{returned_data::ReturnedData, Kiln};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Account {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
}


#[derive(Deserialize, Serialize)]
struct AccountPutRequest {
    name: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
struct AccountPosttRequest {
    name: String,
    description: String,
}

#[derive(Clone, Debug)]
pub struct KilnAccountClient {
    bearer_token: String,
    base_url: String,
}

impl KilnAccountClient {
    pub fn new(kiln: &Kiln) -> Self {
        let url: String = format!("{}/accounts", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }

    pub fn get_by_uuid(&self, id: uuid::Uuid) -> ReturnedData<Account> {
        let id_param = id.to_string();

        let url: String = format!("{}/{}", self.base_url, id_param);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Account>>()
            .unwrap();

        data
    }

    pub fn get_all(&self) -> ReturnedData<Vec<Account>> {
        let data = ureq::get(&self.base_url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Vec<Account>>>()
            .unwrap();

        data
    }

    pub fn post(&self, name: &str, description: &str) -> ReturnedData<Account> {
        let send_body = AccountPosttRequest {
            name: name.into(),
            description: description.into(),
        };

        let data = ureq::post(&self.base_url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(&send_body)
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Account>>()
            .unwrap();

        data
    }

    pub fn put(&self, id: uuid::Uuid, name: &str, description: &str) -> ReturnedData<Account> {
        let id_param = id.to_string();

        let url: String = format!("{}/{}", self.base_url, id_param);

        let send_body = AccountPutRequest {
            name: name.into(),
            description: description.into(),
        };

        let data = ureq::put(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(&send_body)
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Account>>()
            .unwrap();

        data
    }

    pub fn delete(&self, id: uuid::Uuid) -> ReturnedData<Account> {
        let id_param = id.to_string();

        let url: String = format!("{}/{}", self.base_url, id_param);

        let data = ureq::delete(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Account>>()
            .unwrap();

        data
    }

    pub fn portofolio(&self, id: uuid::Uuid, refresh: bool) -> ReturnedData<Account> {
        let id_param: String = id.to_string();

        let refresh_query_string: String = refresh.to_string();

        let url: String = format!(
            "{}/{}/portofolio?refresh={}",
            self.base_url, id_param, refresh_query_string
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Account>>()
            .unwrap();

        data
    }

    pub fn reports(&self, id: uuid::Uuid) -> Vec<u8> {
        let id_param: String = id.to_string();

        let url: String = format!("{}/{}/reports", self.base_url, id_param);

        let mut file_bytes: Vec<u8> = Vec::new();

        ureq::get(url)
            .header("accept", "application/octet-stream")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .as_reader()
            .read_to_end(&mut file_bytes)
            .unwrap();

        file_bytes
    }
}

#[cfg(test)]
mod accounts_test {

    use std::str::FromStr;

    use super::*;


    // the #[ignore] are here because the temporary free api token isn't valid anymore

    #[test]
    #[ignore]
    fn account_get_by_uuid() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let uuid = uuid::Uuid::from_str("9e1c62a4-2b01-4cd8-a2a4-74c57dcd2be4").unwrap();

        let data = kiln.accounts().get_by_uuid(uuid);

        dbg!(data);
    }

    #[test]
    #[ignore]
    fn account_get_all() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let data = kiln.accounts().get_all();

        dbg!(data);
    }

    #[test]
    #[ignore]
    fn account_put() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let uuid = uuid::Uuid::from_str("9e1c62a4-2b01-4cd8-a2a4-74c57dcd2be4").unwrap();

        let data = kiln.accounts().put(uuid, "John", "John John");

        dbg!(data);
    }

    #[test]
    #[ignore]
    fn account_post() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let data = kiln.accounts().post("Tanjiro", "Kamado");

        dbg!(data);
    }

    // IGNORING because no portofolio exist on the dev environment
    #[test]
    #[ignore]
    fn account_portofolio() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let uuid = uuid::Uuid::from_str("9e1c62a4-2b01-4cd8-a2a4-74c57dcd2be4").unwrap();

        let data = kiln.accounts().portofolio(uuid, false);

        dbg!(data);
    }

    // IGNORING because no portofolio exist on the dev environment
    #[test]
    #[ignore]
    fn account_reports() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let uuid = uuid::Uuid::from_str("9e1c62a4-2b01-4cd8-a2a4-74c57dcd2be4").unwrap();

        let data = kiln.accounts().reports(uuid);

        dbg!(data);
    }
}
