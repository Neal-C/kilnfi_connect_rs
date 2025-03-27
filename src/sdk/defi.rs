use crate::{response_format::ReturnedData, Kiln};

use super::{NetworkStats, Operations, Stakes};

pub struct KilnDefiClient {
    bearer_token: String,
    base_url: String,
}

impl From<&Kiln> for KilnDefiClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/defi", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnDefiClient {
    pub fn stakes(
        &self,
        wallets: Vec<String>,
        vaults: Vec<String>,
    ) -> Result<ReturnedData<Vec<Stakes>>, ureq::Error> {
        let wallets = wallets.join(",");

        let vaults = vaults.join(",");

        let url: String = format!(
            "{}/stakes?wallets={}&vaults={}",
            self.base_url, wallets, vaults
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<Stakes>>>();

        data
    }

    pub fn get_operations(&self) -> Result<ReturnedData<Vec<Operations>>, ureq::Error> {
        let url: String = format!("{}/operations", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<Operations>>>();

        data
    }

    pub fn network_stats(&self) -> Result<ReturnedData<Vec<NetworkStats>>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<NetworkStats>>>();

        data
    }
}

#[cfg(test)]
mod defi_test {

    use super::*;

    // the #[ignore] are here because the temporary free api token isn't valid anymore

    // 422 responses
    #[test]
    #[ignore]
    fn defi_stakes() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let data = kiln
            .defi()
            .stakes(
                vec![String::from("")],
                vec![
                    String::from("eth"),
                    String::from("arb"),
                    String::from("bsc"),
                    String::from("matic"),
                    String::from("op"),
                ],
            )
            .unwrap();

        dbg!(data);
    }

    // 422 responses
    #[test]
    #[ignore]
    fn defi_operations() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let data = kiln.defi().get_operations().unwrap();

        dbg!(data);
    }
    // 422 responses
    #[test]
    #[ignore]
    fn network_stats() {
        let api_token: String = std::env::var("KILN_API_TOKEN")
            .expect("KILN_API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url("https://api.kiln.fi/v1")
            .seal()
            .build()
            .unwrap();

        let data = kiln.defi().network_stats().unwrap();

        dbg!(data);
    }
}
