use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::ethereum::ValidationKeys;

// use super::PostValidationKeys;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthOnchainKeysRequest {
    pub account_id: Uuid,
    pub number: u64,
    pub fee_recipient_contract_address: String,
    pub staking_contract_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthSuiteKeysRequest {
    pub account_id: Uuid,
    pub number: u64,
    // Address
    pub factory_contract_address: String,
}

// --- Onchain V2 stakes --- start

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakeStructure {
    pub pool: String,
    // Address
    pub pool_address: String,
    pub share: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StakeStats {
    pub nrr: f64,
    pub grr: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2StakesResponse {
    // Address
    pub owner: String,
    pub integration: String,
    // Address
    pub integration_address: String,
    pub balance: String,
    pub rewards: String,
    pub nrr: f64,
    pub grr: f64,
    pub one_year: StakeStats,
    pub six_months: StakeStats,
    pub three_months: StakeStats,
    pub one_month: StakeStats,
    pub one_week: StakeStats,
    pub structure: Vec<StakeStructure>,
    pub delegated_block: u64,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// --- Onchain V2 stakes --- end

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2OperationsRequest {
    pub wallets: Vec<String>,
    pub ids: Vec<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Unfulfillable,
    Fulfillable,
    PartiallyFulfillable,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum OnchainV2Operation {
    Exit {
        r#type: String,
        ticket_id: String,
        ticket_status: TicketStatus,
        size: String,
        size_shares: String,
        claimable: String,
        claimable_shares: String,
        cask_ids: Vec<String>,
        id: String,
        // Address
        owner: String,
        time: String,
        block: u64,
        tx_hash: String,
    },
    Claim {
        r#type: String,
        ticket_id: String,
        ticket_status: TicketStatus,
        claimed: String,
        claimable_shares: String,
        remaining: String,
        remaining_shares: String,
        used_cask_ids: Vec<String>,
        id: String,
        // Address
        owner: String,
        time: String,
        block: u64,
        tx_hash: String,
    },
    Deposit {
        r#type: String,
        amount: String,
        amount_shares: String,
        id: String,
        // Address
        owner: String,
        time: String,
        block: u64,
        tx_hash: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2RewardsRequest {
    pub wallets: Vec<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub include_usd: bool,
    // Address
    pub integration: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2Reward {
    pub date: chrono::DateTime<chrono::Utc>,
    pub rewards: String,
    pub balance: String,
    pub nrr: f64,
    pub rewards_usd: u64,
    pub balance_usd: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2NetworkStatsRequest {
    // Address
    pub integration: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Pool {
    // Address
    pub address: String,
    pub name: String,
    pub ratio: u64,
    pub commission: u64,
    pub total_deposited: String,
    // Address
    pub factory_address: String,
    // Address
    pub operator_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2NetworkStatsResponse {
    // Address
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub fee: u64,
    pub total_supply: String,
    pub total_underlying_supply: String,
    pub total_stakers: u64,
    pub nrr: f64,
    pub grr: f64,
    pub one_year: StakeStats,
    pub six_months: StakeStats,
    pub three_months: StakeStats,
    pub one_month: StakeStats,
    pub one_week: StakeStats,
    pub pools: Vec<Pool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OnchainV2ExitTicketsResponse {
    // Address
    pub owner: String,
    pub state: TicketStatus,
    pub retrievable_amount: String,
    pub exiting_amount: String,
    // Address
    pub exit_queue_address: String,
    // Address
    pub integration_address: String,
    pub ticket_id: String,
    pub cask_id: Option<String>,
    pub estimated_claimable_at: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct KilnEthereumOnchainClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnEthereumOnchainClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/eth/onchain", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnEthereumOnchainClient {
    pub fn post_eth_onchain_keys(
        &self,
        eth_onchain_keys_request: &EthOnchainKeysRequest,
    ) -> Result<ReturnedData<Vec<ValidationKeys>>, ureq::Error> {
        let url = format!("{}/v1/keys", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(eth_onchain_keys_request)?
            .body_mut()
            .read_json::<ReturnedData<Vec<ValidationKeys>>>();

        data
    }

    pub fn post_eth_suite_keys(
        &self,
        eth_onchain_keys_request: &EthSuiteKeysRequest,
    ) -> Result<ReturnedData<Vec<ValidationKeys>>, ureq::Error> {
        let url = format!("{}/v2/keys", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(eth_onchain_keys_request)?
            .body_mut()
            .read_json::<ReturnedData<Vec<ValidationKeys>>>();

        data
    }

    pub fn get_onchain_v2_stakes(
        &self,
        wallets: Vec<String>,
    ) -> Result<ReturnedData<OnchainV2StakesResponse>, ureq::Error> {
        let wallets = wallets.join(",");

        let url: String = format!("{}/v2/stakes?&wallets={}", self.base_url, wallets);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<OnchainV2StakesResponse>>();

        data
    }

    pub fn get_onchain_v2_operations(
        &self,
        onchain_v2_operation_request: &OnchainV2OperationsRequest,
    ) -> Result<ReturnedData<Vec<OnchainV2Operation>>, ureq::Error> {
        let OnchainV2OperationsRequest {
            wallets,
            ids,
            start_date,
            end_date,
        } = onchain_v2_operation_request;

        let wallets = wallets.join(",");

        let ids = ids.join(",");

        let url: String = format!(
            "{}/v2/operations?&wallets={}&ids={}&start_date={}&end_date={}",
            self.base_url, wallets, ids, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<OnchainV2Operation>>>();

        data
    }

    pub fn get_onchain_v2_rewards(
        &self,
        onchain_v2_rewards_request: &OnchainV2RewardsRequest,
    ) -> Result<ReturnedData<Vec<OnchainV2Reward>>, ureq::Error> {
        let OnchainV2RewardsRequest {
            wallets,
            start_date,
            end_date,
            integration,
            include_usd,
        } = onchain_v2_rewards_request;

        let wallets = wallets.join(",");

        let url: String = format!(
            "{}/v2/rewards?&wallets={}&start_date={}&end_date={}&include_usd={}&integration={}",
            self.base_url, wallets, start_date, end_date, include_usd, integration
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<OnchainV2Reward>>>();

        data
    }

    pub fn get_onchain_v2_network_stats(
        &self,
        integration_request: &OnchainV2NetworkStatsRequest,
    ) -> Result<ReturnedData<OnchainV2NetworkStatsResponse>, ureq::Error> {
        let url: String = format!(
            "{}/v2/network-stats?&integration={}",
            self.base_url, integration_request.integration,
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<OnchainV2NetworkStatsResponse>>();

        data
    }

    pub fn get_onchain_v2_exit_tickets(
        &self,
        wallets: Vec<String>,
    ) -> Result<ReturnedData<Vec<OnchainV2ExitTicketsResponse>>, ureq::Error> {
        let wallets = wallets.join(",");

        let url: String = format!("{}/v2/exit-tickets?&wallets={}", self.base_url, wallets);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<OnchainV2ExitTicketsResponse>>>();

        data
    }
}
