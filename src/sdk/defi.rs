use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

use crate::{returned_data::ReturnedData, Kiln};

#[derive(Deserialize, Serialize, AsRefStr, Debug, Clone)]
#[non_exhaustive]
pub enum DefiProtocol {
    AaveV3,
    Venus,
    CompoundV3,
    Morpho,
    Sdai,
}

#[derive(Deserialize, Serialize, AsRefStr, Debug, Clone)]
#[non_exhaustive]
pub enum Operation {
    Deposit,
    Withdrawal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkStats {
    asset: String,
    asset_icon: String,
    asset_symbol: String,
    asset_decimals: String,
    assets_price_usd: String,
    share_symbol: String,
    tvl: String,
    protocol: String,
    protocol_display_name: String,
    protocol_icon: String,
    protocol_tvl: String,
    protocol_supply_limit: String,
    // Gross Reward Rate
    grr: u64,
    // Net Reward Rate
    nrr: u64,
    vault: String,
    chain: String,
    chain_id: u64,
    updated_at_block: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stakes {
    // Address
    owner: String,
    current_balance: String,
    total_rewards: String,
    current_rewards: String,
    total_deposited_amount: String,
    total_withdrawn_amount: String,
    // Address
    vault: String,
    chain: String,
    updated_at_block: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Operations {
    // Address
    #[serde(rename(serialize = "type", deserialize = "type"))]
    operation_type: Operation,
    owner: String,
    assets: String,
    shares: String,
    sender: String,
    // Operation timestamp (RFC3339 format)
    timestamp: String,
    tx_hash: String,
    vault: String,
    chain: String,
}

#[derive(Clone, Debug)]
pub struct KilnDefiClient {
    bearer_token: String,
    base_url: String,
}

impl KilnDefiClient {
    pub fn new(kiln: &Kiln) -> Self {
        let url: String = format!("{}/defi", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }

    pub fn stakes(&self, wallets: Vec<String>, vaults: Vec<String>) -> ReturnedData<Vec<Stakes>> {
        let wallets_query = wallets.join(",");
        let vaults_query = vaults.join(",");
        let url: String = format!(
            "{}/stakes?wallets={}&vaults={}",
            self.base_url, wallets_query, vaults_query
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Vec<Stakes>>>()
            .unwrap();

        data
    }

    pub fn operations(&self) -> ReturnedData<Vec<Operations>> {
        let url: String = format!("{}/operations", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Vec<Operations>>>()
            .unwrap();

        data
    }

    pub fn network_stats(&self) -> ReturnedData<Vec<NetworkStats>> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ReturnedData<Vec<NetworkStats>>>()
            .unwrap();

        data
    }
}

#[cfg(test)]
mod defi_test {

    use super::*;

    // DELETE Method isn't tested as to not bother other hackathon participants

    // 422 responses
    #[test]
    #[ignore]
    fn defi_stakes() {
        let api_token: String = std::env::var("API_TOKEN")
            .expect("API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url(String::from("https://api.kiln.fi/v1"))
            .seal()
            .build()
            .unwrap();

        let data = kiln.defi().stakes(
            vec![String::from("")],
            vec![
                String::from("eth"),
                String::from("arb"),
                String::from("bsc"),
                String::from("matic"),
                String::from("op"),
            ],
        );

        dbg!(data);
    }

    // 422 responses
    #[test]
    #[ignore]
    fn defi_operations() {
        let api_token: String = std::env::var("API_TOKEN")
            .expect("API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url(String::from("https://api.kiln.fi/v1"))
            .seal()
            .build()
            .unwrap();

        let data = kiln.defi().operations();

        dbg!(data);
    }
    // 422 responses
    #[test]
	#[ignore]
    fn network_stats() {
        let api_token: String = std::env::var("API_TOKEN")
            .expect("API_TOKEN is not set in the environment. It is required.");

        let kiln: Kiln = Kiln::builder()
            .api_token(api_token)
            // no trailing slash, or else it will break
            .base_url(String::from("https://api.kiln.fi/v1"))
            .seal()
            .build()
            .unwrap();

        let data = kiln.defi().network_stats();

        dbg!(data);
    }
}
