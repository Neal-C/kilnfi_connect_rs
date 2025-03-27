use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxRequest, BroadcastTxResponse, PrepareTxResponse, RewardRequest, StakeState,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosGetStakesRequest {
    // Addresses
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub validators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosStakesResponse {
    // Addresses
    pub stakes_addresses: Vec<String>,
    pub baker_address: String,
    pub state: StakeState,
    pub stake_type: String,
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub activated_cycle: u64,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub delegated_cycle: u64,
    pub delegated_block: String,
    pub undelegated_at: chrono::DateTime<chrono::Utc>,
    pub undelegated_cycle: u64,
    pub balance: String,
    pub staked_balance: String,
    pub wallet_balance: String,
    pub rewards: String,
    pub staked_rewards: String,
    pub gross_apy: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TezosXTZReward {
    ByDay {
        date: chrono::DateTime<chrono::Utc>,
        rewards: String,
        active_balance: String,
        staked_rewards: String,
        staked_balance: String,
        gross_apy: f64,
        rewards_usd: String,
        stake_balance_usd: String,
        active_balance_usd: Option<u64>,
        staked_balance_usd: u64,
    },
    ByCycle {
        cycle: u64,
        cycle_begins_at: chrono::DateTime<chrono::Utc>,
        rewards: String,
        active_balance: String,
        staked_rewards: String,
        staked_balance: String,
        gross_apy: f64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosOperationsRequest {
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub validators: Vec<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TezosOperation {
    Delegate {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        operation: String,
        operation_gas_used: String,
        baker_fee: String,
        block: u64,
        amount: String,
    },
    Undelegate {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        operation: String,
        operation_gas_used: String,
        baker_fee: String,
        block: u64,
    },
    Activation {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        cycle: u64,
    },
    Payment {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        // Address
        sender_address: String,
        operation: String,
        operation_gas_used: String,
        baker_fee: String,
        block: u64,
        amount: String,
    },
    Stake {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        operation: String,
        operation_gas_used: String,
        baker_fee: String,
        block: u64,
        amount: String,
    },
    Unstake {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        operation: String,
        operation_gas_used: String,
        baker_fee: String,
        block: u64,
        amount: String,
    },
    Finalize {
        r#type: String,
        date: chrono::DateTime<chrono::Utc>,
        // Address
        staker_address: String,
        // Address
        baker_address: String,
        operation: String,
        operation_gas_used: String,
        baker_fee: String,
        block: u64,
        amount: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosNetworkStatsResponse {
    pub xtz_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosReportsRequest {
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosDelegateTxRequest {
    pub account_id: Uuid,
    // Address
    pub wallet: String,
    // Address
    pub baker_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxParametersValue {
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxContentParamaters {
    pub entrypoint: String,
    pub value: TxParametersValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosBalanceUpdate {
    pub kind: String,
    pub contract: String,
    pub change: String,
    pub origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TezosTxArg {
    Int { prim: String, int: String },
    String { prim: String, string: String },
    Bytes { prim: String, bytes: String },
    Args { prim: String, args: Vec<TezosTxArg> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxStorage {
    pub prim: String,
    pub args: Vec<Vec<TezosTxArg>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosDiffUpdateKey {
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosDiffUpdate {
    pub key_hash: String,
    pub key: TezosDiffUpdateKey,
    pub value: Vec<Vec<TezosTxArg>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosDiff {
    pub action: String,
    pub updates: Vec<TezosDiffUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosLazyStorageDiff {
    pub kind: String,
    pub id: String,
    pub diff: TezosDiff,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosOperationResult {
    pub status: TezosTxStatus,
    pub storage: TezosTxStorage,
    pub balance_updates: Vec<TezosBalanceUpdate>,
    pub consumed_milligas: String,
    pub storage_size: String,
    pub paid_storage_size_diff: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxMetadata {
    pub balance_updates: Vec<TezosBalanceUpdate>,
    pub operation_result: TezosOperationResult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxContent {
    pub kind: String,
    pub source: String,
    pub fee: String,
    pub counter: String,
    pub gas_limit: String,
    pub storage_limit: String,
    // Address
    pub delegate: String,
    pub destination: String,
    pub parameters: TxContentParamaters,
    pub metadata: TezosTxMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosUnsignedTx {
    pub branch: String,
    pub contents: Vec<TezosTxContent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxResponse {
    pub unsigned_tx_hash: String,
    pub unsigned_tx_serialized: String,
    pub unsigned_tx: TezosUnsignedTx,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosUndelegateTxRequest {
    // Address
    pub wallet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosFinalizeUnstakeTxRequest {
    // Address
    pub wallet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosStakeTxRequest {
    // Address
    pub wallet: String,
    pub amount_umutez: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosUnstakeTxRequest {
    // Address
    pub wallet: String,
    pub amount_umutez: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxStatusRequest {
    pub tx_hash: String,
    pub block: u64,
}

#[derive(Serialize, Deserialize, Debug, EnumString, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum TezosTxStatus {
    Applied,
    Failed,
    Skipped,
    Backtracked,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosStatusReceipt {
    pub protocol: String,
    // Looks more like a string than a ChainID
    pub chain_id: String,
    pub hash: String,
    pub branch: String,
    pub contents: Vec<TezosTxContent>,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxStatusResponse {
    pub status: TezosTxStatus,
    pub receipt: TezosStatusReceipt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TezosTxDecodingResponse {
    pub branch: String,
    pub contents: Vec<TezosTxContent>,
}

#[derive(Clone, Debug)]
pub struct KilnTezosClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnTezosClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/xtz", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnTezosClient {
    pub fn get_stakes(
        &self,
        stakes_request: &TezosGetStakesRequest,
    ) -> Result<ReturnedData<Vec<TezosStakesResponse>>, ureq::Error> {
        let TezosGetStakesRequest {
            validators,
            wallets,
            accounts,
        } = stakes_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/stakes?validators={}&wallets={}&accounts={}",
            self.base_url, validators, wallets, accounts
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<TezosStakesResponse>>>();

        data
    }

    pub fn get_rewards(
        &self,
        reward_request: RewardRequest,
    ) -> Result<ReturnedData<Vec<TezosXTZReward>>, ureq::Error> {
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
            .read_json::<ReturnedData<Vec<TezosXTZReward>>>();

        data
    }

    pub fn get_operations(
        &self,
        validator_operations_request: &TezosOperationsRequest,
    ) -> Result<ReturnedData<Vec<TezosOperation>>, ureq::Error> {
        let TezosOperationsRequest {
            validators,
            wallets,
            accounts,
            start_date,
            end_date,
        } = validator_operations_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/operations?validators={}&wallets={}&accounts={}&start_date={}&end_date={}",
            self.base_url, validators, wallets, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<TezosOperation>>>();

        data
    }

    pub fn get_network_stats(
        &self,
    ) -> Result<ReturnedData<TezosNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<TezosNetworkStatsResponse>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: &TezosReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let TezosReportsRequest { wallets, accounts } = reports_request;

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/reports?wallets={}&accounts={}",
            self.base_url, wallets, accounts
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

    pub fn post_delegate_tx(
        &self,
        delegate_tx_request: &TezosDelegateTxRequest,
    ) -> Result<ReturnedData<TezosTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/delegate", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(delegate_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<TezosTxResponse>>();

        data
    }

    pub fn post_undelegate_tx(
        &self,
        undelegate_tx_request: &TezosUndelegateTxRequest,
    ) -> Result<ReturnedData<TezosTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/undelegate", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(undelegate_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<TezosTxResponse>>();

        data
    }

    pub fn post_stake_tx(
        &self,
        transaction_stake_request: &TezosStakeTxRequest,
    ) -> Result<ReturnedData<TezosTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/stake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(transaction_stake_request)?
            .body_mut()
            .read_json::<ReturnedData<TezosTxResponse>>();

        data
    }

    pub fn post_unstake_tx(
        &self,
        transaction_unstake_request: &TezosUnstakeTxRequest,
    ) -> Result<ReturnedData<TezosTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/unstake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(transaction_unstake_request)?
            .body_mut()
            .read_json::<ReturnedData<TezosTxResponse>>();

        data
    }

    pub fn post_finalize_unstake_tx(
        &self,
        finalize_unstake_tx_request: &TezosFinalizeUnstakeTxRequest,
    ) -> Result<ReturnedData<TezosTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/finalize-unstake", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(finalize_unstake_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<TezosTxResponse>>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &TezosPrepareTxRequest,
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
        tx_status_request: &TezosTxStatusRequest,
    ) -> Result<ReturnedData<TezosTxStatusResponse>, ureq::Error> {
        let TezosTxStatusRequest { block, tx_hash } = tx_status_request;

        let url: String = format!(
            "{}/transaction/status?tx_hash={}&block={}",
            self.base_url, tx_hash, block
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<TezosTxStatusResponse>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<ReturnedData<TezosTxDecodingResponse>, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<TezosTxDecodingResponse>>();

        data
    }
}
