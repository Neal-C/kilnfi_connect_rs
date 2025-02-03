use std::io::Read;

use crate::{returned_data::ReturnedData, Kiln};

use super::Portofolio;

#[derive(Clone, Debug)]
pub struct KilnOrganisationClient {
    bearer_token: String,
    base_url: String,
}

impl KilnOrganisationClient {
    pub fn new(kiln: &Kiln) -> Self {
        let url: String = format!("{}/organisations", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }

    pub fn get_by_uuid(&self, id: uuid::Uuid) -> ReturnedData<Portofolio> {
        let id_param = id.to_string();

        let url: String = format!("{}/{}", self.base_url, id_param);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Portofolio>>()
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
mod organisations_test {

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

        let data = kiln.organisations().get_by_uuid(uuid);

        dbg!(data);
    }
}
