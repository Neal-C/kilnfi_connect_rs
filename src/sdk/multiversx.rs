use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::ReportsRequest;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxGetStakesRequest {
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub validators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxStakesResponse {
    pub wallet: String,
    pub validator: String,
    pub state: String,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub delegated_epoch: u64,
    pub undelegated_at: chrono::DateTime<chrono::Utc>,
    pub undelegated_epoch: u64,
    pub balance: String,
    pub rewards: String,
    pub grr: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxRewardRequest {
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub validators: Vec<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxReward {
    pub date: chrono::DateTime<chrono::Utc>,
    pub rewards: String,
    pub active_balance: String,
    pub grr: f64,
    pub rewards_usd: u64,
    pub active_balance_usd: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxNetworkStatsResponse {
    pub egld_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxOperationsRequest {
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub validators: Vec<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum MultiversxOperationType {
    #[strum(serialize = "addNodes")]
    AddNodes,
    #[strum(serialize = "changeOwner")]
    ChangeOwner,
    #[strum(serialize = "changeServiceFee")]
    ChangeServiceFee,
    #[strum(serialize = "getAllNodeStates")]
    GetAllNodeStates,
    #[strum(serialize = "modifyTotalDelegationCap")]
    ModifyTotalDelegationCap,
    #[strum(serialize = "removeNodes")]
    RemoveNodes,
    #[strum(serialize = "reStakeUnstakedNodes")]
    RestakeUnstakedNodes,
    #[strum(serialize = "setAutomaticActivation")]
    SetAutomaticActivation,
    #[strum(serialize = "setCheckCapOnReDelegateRewards")]
    SetCheckCapOnReDelegateRewards,
    #[strum(serialize = "setMetaData")]
    SetMetaData,
    #[strum(serialize = "stakeNodes")]
    StakeNodes,
    #[strum(serialize = "synchronizeOwner")]
    SynchronizeOwner,
    #[strum(serialize = "unBondNodes")]
    UnBondNodes,
    #[strum(serialize = "unJailNodes")]
    UnJailNodes,
    #[strum(serialize = "delegate")]
    Delegate,
    #[strum(serialize = "unStakeNodes")]
    UnStakeNodes,
    #[strum(serialize = "whitelistForMerge")]
    WhitelistForMerge,
    #[strum(serialize = "undelegate")]
    UnDelegate,
    #[strum(serialize = "claimRewards")]
    ClaimRewards,
    #[strum(serialize = "reDelegateRewards")]
    RedelegateRewards,
    #[strum(serialize = "reward")]
    Reward,
    #[strum(serialize = "withdraw")]
    Withdraw,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MultiversxOperation {
    // Address (EGLD address)
    pub sender: String,
    // Address (EGLD address)
    pub receiver: String,
    pub tx_hash: String,
    pub tx_timestamp: chrono::DateTime<chrono::Utc>,
    pub amount: String,
    pub gas_used: String,
    pub r#type: MultiversxOperationType,
}

#[derive(Clone, Debug)]
pub struct KilnMultiversxClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnMultiversxClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/egld", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnMultiversxClient {
    pub fn get_stakes(
        &self,
        stakes_request: &MultiversxGetStakesRequest,
    ) -> Result<ReturnedData<Vec<MultiversxStakesResponse>>, ureq::Error> {
        let MultiversxGetStakesRequest {
            wallets,
            accounts,
            validators,
        } = stakes_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/stakes?validators={}&delegators={}&accounts={}",
            self.base_url, validators, wallets, accounts
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<MultiversxStakesResponse>>>();

        data
    }

    pub fn get_rewards(
        &self,
        reward_request: &MultiversxRewardRequest,
    ) -> Result<ReturnedData<Vec<MultiversxReward>>, ureq::Error> {
        let MultiversxRewardRequest {
            validators,
            wallets,
            accounts,
            start_date,
            end_date,
        } = reward_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/rewards?validators={}&delegators={}&accounts={}&start_date={}&end_date={}",
            self.base_url, validators, wallets, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<MultiversxReward>>>();

        data
    }

    pub fn get_network_stats(
        &self,
    ) -> Result<ReturnedData<MultiversxNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<MultiversxNetworkStatsResponse>>();

        data
    }

    pub fn get_operations(
        &self,
        validator_operations_request: &MultiversxOperationsRequest,
    ) -> Result<ReturnedData<Vec<MultiversxOperation>>, ureq::Error> {
        let MultiversxOperationsRequest {
            wallets,
            validators,
            accounts,
            start_date,
            end_date,
        } = validator_operations_request;

        let wallets = wallets.join(",");

        let validators = validators.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/operations?wallets={}&validators={}&accounts={}&start_date={}&end_date={}",
            self.base_url, wallets, validators, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<MultiversxOperation>>>();

        data
    }

    pub fn get_reports(&self, reports_request: &ReportsRequest) -> Result<Vec<u8>, ureq::Error> {
        let ReportsRequest {
            delegators,
            validators,
            accounts,
        } = reports_request;

        let delegators = delegators.join(",");

        let validators = validators.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/reports?validators={}&delegators={}&accounts={}",
            self.base_url, validators, delegators, accounts
        );

        let mut file_bytes: Vec<u8> = Vec::new();

        ureq::get(url)
            .header("accept", "application/octet-stream")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .as_reader()
            .read_to_end(&mut file_bytes)?;

        Ok(file_bytes)
    }
}
