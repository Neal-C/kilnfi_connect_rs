use std::io::Read;

use serde::{Deserialize, Deserializer, Serialize};
use strum_macros::EnumString;
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxRequest, BroadcastTxResponse, Chain, ChainID, GetBalanceRequest, GetStakesRequest,
    PostStakesRequest, PostStakesResponse, PrepareTxRequest, PrepareTxResponse, Protocol,
    ReportsRequest, RewardRequest, TxDecodingResponse, TxResponse, TxStakeCoin, TxStatusResponse,
    WithdrawRewardsTxRequest,
};

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub struct DydxGetStakesRequest {
//     pub delegators: Vec<String>,
//     pub validators: Vec<String>,
//     pub accounts: Vec<Uuid>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum DydxStakeState {
    Active,
    Inactive,
    Deactivating,
}

#[derive(Serialize, Deserialize, EnumString, Debug, Default)]
pub enum DydxStakePermissionKind {
    #[default]
    #[strum(serialize = "Staking.MsgDelegate")]
    Delegate,
    #[strum(serialize = "Staking.MsgUndelegate")]
    Undelegate,
    #[strum(serialize = "Staking.Redelegate")]
    Redelegate,
    #[strum(serialize = "Distribution.MsgWithdrawDelegatorReward")]
    WithdrawDelegatorReward,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxGetStakesResponsePermission {
    pub source: String,
    pub creation_height: u64,
    pub permission: DydxStakePermissionKind,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub allow_list: Option<Vec<String>>,
    pub deny_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxGetStakesResponseUnbonding {
    pub creation_height: u64,
    pub completion_time: chrono::DateTime<chrono::Utc>,
    pub balance: String,
    pub initial_balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxGetStakesResponse {
    pub validator_address: String,
    pub delegator_address: String,
    pub delegated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub delegated_block: Option<u64>,
    pub undelegated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub undelegated_block: Option<u64>,
    pub rewards: String,
    pub available_rewards: String,
    pub balance: String,
    pub net_apy: f64,
    pub state: DydxStakeState,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub permissions: Vec<DydxGetStakesResponsePermission>,
    pub unbondings: Vec<DydxGetStakesResponseUnbonding>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub struct DydxCreateStakeRequestStake {
//     pub stake_id: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub struct DydxCreateStakeRequest {
//     pub stakes: Vec<DydxCreateStakeRequestStake>,
//     pub account_id: Uuid,
// }

#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum DydxStakeStatus {
    #[strum(serialize = "active")]
    Active,
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "disabled")]
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxCreateStakeResponse {
    pub id: Uuid,
    pub tags: Vec<String>,
    pub metada: serde_json::Value,
    pub protocol: Protocol,
    pub chain_id: ChainID,
    pub chain: Chain,
    pub address: String,
    pub status: DydxStakeStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxReward {
    pub date: chrono::DateTime<chrono::Utc>,
    pub rewards: String,
    pub balance: String,
    pub net_apy: f64,
    pub rewards_uusdc: String,
    pub rewards_usdc_usd: String,
    pub rewards_usd: String,
    pub balance_usd: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxOperationsRequest {
    pub validators: Vec<String>,
    pub delegators: Vec<String>,
    pub authz: bool,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum DydxOperation {
    #[strum(serialize = "Staking.MsgDelegate", serialize = "staking.MsgDelegate")]
    Delegate {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
    },
    #[strum(
        serialize = "Staking.MsgUndelegate",
        serialize = "staking.MsgUndelegate"
    )]
    Undelegate {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
    },
    #[strum(
        serialize = "Staking.MsgBeginRedelegate",
        serialize = "staking.MsgBeginRedelegate"
    )]
    BeginRedelegate {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        validator_address_source: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
        withdraw_rewards_source: String,
    },
    #[strum(
        serialize = "Distribution.MsgWithdrawDelegatorReward",
        serialize = "distribution.MsgWithdrawDelegatorReward"
    )]
    WithdrawDelegatorReward {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        validator_address: String,
        delegator_address: String,
        withdraw_rewards: String,
    },
    Grant {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        block: u64,
        tx_hash: String,
        tx_gas_used: String,
        message_index: u64,
        grantee: String,
        granter: String,
        permission: DydxStakePermissionKind,
        allow_list: Option<Vec<String>>,
        denly_list: Option<Vec<String>>,
    },
    Exec {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        validator_address: String,
        validator_address_source: String,
        delegator_address: String,
        amount: String,
        withdraw_rewards: String,
        withdraw_rewards_source: String,
        executed_operations: Option<Vec<DydxOperation>>,
    },
}

fn deserialize_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let the_type = DydxOperation::deserialize(deserializer)?;

    let the_type = match the_type {
        DydxOperation::Delegate { .. } => "staking.MsgDelegate".into(),
        DydxOperation::Undelegate { .. } => "staking.MsgUndelegate".into(),
        DydxOperation::WithdrawDelegatorReward { .. } => "distr.MsgWithdrawDelegatorReward".into(),
        DydxOperation::BeginRedelegate { .. } => "staking.MsgBeginRedelegate".into(),
        DydxOperation::Grant { .. } => "auth.grant".into(),
        DydxOperation::Exec { .. } => "auth.exec".into(),
    };

    Ok(the_type)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxNetworkStatsResponse {
    pub dydx_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxStakeTxRequest {
    pub account_id: Uuid,
    pub pubkey: String,
    pub validator: String,
    pub amount_adydx: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxUnstakeTxRequest {
    // Pubkey
    pub pubkey: String,
    // Address
    pub validator: String,
    pub amount_udydx: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DydxRedelegateTxRequest {
    pub account_id: Uuid,
    // Pubkey
    pub pubkey: String,
    pub validator_source: String,
    pub validar_destination: String,
    pub amount_udydx: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NobleIBCTransferRequest {
    pub pubkey: String,
    pub amount_uusdc: String,
}

#[derive(Clone, Debug)]
pub struct KilnDydxClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnDydxClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/dydx", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnDydxClient {
    pub fn get_stakes(
        &self,
        dydx_stake_request: &GetStakesRequest,
    ) -> Result<ReturnedData<DydxGetStakesResponse>, ureq::Error> {
        let GetStakesRequest {
            validators,
            delegators,
            accounts,
        } = dydx_stake_request;

        let validators = validators.join(",");

        let delegators = delegators.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/stakes?validators={}&delegators={}&accounts={}",
            self.base_url, validators, delegators, accounts
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<DydxGetStakesResponse>>();

        data
    }

    pub fn post_stakes(
        &self,
        post_stakes_request: &PostStakesRequest,
    ) -> Result<ReturnedData<PostStakesResponse>, ureq::Error> {
        let url = format!("{}/stakes", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(post_stakes_request)?
            .body_mut()
            .read_json::<ReturnedData<PostStakesResponse>>();

        data
    }

    pub fn get_rewards(
        &self,
        reward_request: RewardRequest,
    ) -> Result<ReturnedData<Vec<DydxReward>>, ureq::Error> {
        let url: String = match reward_request {
            RewardRequest::Epoch {
                stakes_addresses,
                wallets,
                pool_ids,
                accounts,
                format,
                start_date,
                end_date,
                start_epoch,
                end_epoch,
            } => {
                let stakes_addresses = stakes_addresses.join(",");

                let pool_ids = pool_ids.join(",");

                let wallets = wallets.join(",");

                let accounts = accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let url = format!(
            "{}/rewards?stake_addresses={}&pool_ids={}&wallets={}&accounts={}&format={}&start_date={}&end_date={}&start_epoch={}&end_epoch={}",
            self.base_url,
            stakes_addresses,
            pool_ids,
            wallets,
            accounts,
            format.as_ref(),
            start_date,
            end_date,
            start_epoch,
            end_epoch
        );

                url
            }
            RewardRequest::Daily {
                stakes_addresses,
                wallets,
                pool_ids,
                accounts,
                format,
                start_date,
                end_date,
                include_usd,
            } => {
                let stakes_addresses = stakes_addresses.join(",");

                let pool_ids = pool_ids.join(",");

                let wallets = wallets.join(",");

                let accounts = accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let url = format!(
            "{}/rewards?stake_addresses={}&pool_ids={}&wallets={}&accounts={}&format={}&start_date={}&end_date={}&include_usd={}",
            self.base_url,
            stakes_addresses,
            pool_ids,
            wallets,
            accounts,
            format.as_ref(),
            start_date,
            end_date,
            include_usd
        );

                url
            }
        };

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<DydxReward>>>();

        data
    }

    pub fn get_operations(
        &self,
        dydx_operations_request: &DydxOperationsRequest,
    ) -> Result<ReturnedData<Vec<DydxOperation>>, ureq::Error> {
        let DydxOperationsRequest {
            validators,
            delegators,
            accounts,
            authz,
            start_date,
            end_date,
        } = dydx_operations_request;

        let validators = validators.join(",");

        let delegators = delegators.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/operations?validators={}&authz={}&delegators={}&accounts={}&start_date={}&end_date={}",
            self.base_url, validators, authz, delegators, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<DydxOperation>>>();

        data
    }

    pub fn get_network_stats(&self) -> Result<ReturnedData<DydxNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<DydxNetworkStatsResponse>>();

        data
    }

    pub fn get_reports(
        &self,
        dydx_reports_request: &ReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let ReportsRequest {
            validators,
            delegators,
            accounts,
        } = dydx_reports_request;

        let validators = validators.join(",");

        let delegators = delegators.join(",");

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

    pub fn post_get_balance(
        &self,
        get_balance_request: &GetBalanceRequest,
    ) -> Result<ReturnedData<TxStakeCoin>, ureq::Error> {
        let url: String = format!("{}/balance", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(get_balance_request)?
            .body_mut()
            .read_json::<ReturnedData<TxStakeCoin>>();

        data
    }

    pub fn post_stake_tx(
        &self,
        transaction_stake_request: &DydxStakeTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/stake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(transaction_stake_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();

        data
    }

    pub fn post_withdraw_rewards_tx(
        &self,
        withdraw_rewards_transaction_request: &WithdrawRewardsTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/withdraw-rewards", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_rewards_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();

        data
    }

    pub fn post_unstake_tx(
        &self,
        unstake_rewards_transaction_request: &DydxUnstakeTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/unstake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unstake_rewards_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();
        data
    }

    pub fn post_redelegate_tx(
        &self,
        redelegate_transaction_request: &DydxRedelegateTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/redelegate", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(redelegate_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();
        data
    }

    pub fn noble_ibc_transfer(
        &self,
        noble_ibc_transfer_request: &NobleIBCTransferRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/noble-ibc-transfer", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(noble_ibc_transfer_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();
        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &PrepareTxRequest,
    ) -> Result<ReturnedData<PrepareTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/prepare", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(prepare_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<PrepareTxResponse>>();

        data
    }

    pub fn post_broadcast_tx(
        &self,
        broadcast_transaction_request: &BroadcastTxRequest,
    ) -> Result<ReturnedData<BroadcastTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/broadcast", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(broadcast_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<BroadcastTxResponse>>();

        data
    }

    pub fn get_tx_status(
        &self,
        tx_hash: &str,
    ) -> Result<ReturnedData<TxStatusResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/status?tx_hash={}", self.base_url, tx_hash);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<TxStatusResponse>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<ReturnedData<TxDecodingResponse>, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<TxDecodingResponse>>();

        data
    }
}
