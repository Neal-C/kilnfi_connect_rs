use std::io::Read;
use std::num::NonZeroU64;

use serde::Deserializer;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use strum_macros::EnumString;
use uuid::Uuid;

use crate::response_format::ReturnedData;
use crate::{response_format::PaginatedData, Kiln};

use super::{BroadcastTxRequest, BroadcastTxResponse, ChainID, TxStatusResponse};

#[derive(Serialize, Deserialize, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Kiln,
    Network,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumScopedStakesRequest {
    pub validators: Vec<String>,
    pub scope: Scope,
    pub wallets: Vec<String>,
    pub proxies: Vec<String>,
    pub withdrawal_credentials: Vec<String>,
    pub validator_indexes: Vec<u64>,
    pub include_eigenlayer: bool,
    pub accounts: Vec<Uuid>,
    pub current_page: NonZeroU64,
    pub page_size: NonZeroU64,
}

#[derive(Serialize, Deserialize, EnumString, Debug, AsRefStr)]
pub enum EthereumFilterState {
    #[strum(serialize = "unknown")]
    Unknown,
    #[strum(serialize = "unstaked")]
    Unstaked,
    #[strum(serialize = "deposit_in_progress")]
    DepositInProgress,
    #[strum(serialize = "pending_initialized")]
    PendingInitialized,
    #[strum(serialize = "pending_queued")]
    PendingQueued,
    #[strum(serialize = "active_ongoing")]
    ActiveOngoing,
    #[strum(serialize = "active_exiting")]
    ActiveExiting,
    #[strum(serialize = "active_slashed")]
    ActiveSlashed,
    #[strum(serialize = "exited_unslashed")]
    ExitedUnslashed,
    #[strum(serialize = "exited_slashed")]
    ExitedSlashed,
    #[strum(serialize = "withdrawal_possible")]
    WithdrawalPossible,
    #[strum(serialize = "withdrawal_done")]
    WithdrawalDone,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumNetworkStakesRequest {
    pub validators: Vec<String>,
    pub wallets: Vec<String>,
    pub proxies: Vec<String>,
    pub withdrawal_credentials: Vec<String>,
    pub validator_indexes: Vec<u64>,
    pub include_eigenlayer: bool,
    pub accounts: Vec<Uuid>,
    pub current_page: NonZeroU64,
    pub page_size: u64,
    pub filtered_states: Vec<EthereumFilterState>,
}

#[derive(Debug)]
pub enum EthereumStakesRequest {
    Network(EthereumNetworkStakesRequest),
    Scoped(EthereumScopedStakesRequest),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumEigenlayer {
    pub is_restaked: bool,
    pub is_restakable: bool,
    pub is_pending: bool,
    pub is_withdrawn: bool,
    pub last_checkpointed_at: u64,
    pub points: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumStakesResponse {
    // Address
    pub validator_address: String,
    pub validator_index: String,
    pub state: EthereumFilterState,
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub activated_epoch: u64,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub delegated_block: u64,
    pub exited_at: chrono::DateTime<chrono::Utc>,
    pub exited_epoch: u64,
    // Address
    pub deposit_tx_sender: String,
    // Address
    pub execution_fee_recipient: String,
    pub withdrawal_credentials: String,
    pub effective_balance: String,
    pub balance: String,
    pub consensus_rewards: String,
    pub execution_rewards: String,
    pub rewards: String,
    pub claimable_execution_rewards: String,
    pub claimable_consensus_rewards: String,
    pub gross_apy: f64,
    pub is_kiln: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub eigenlayer: EthereumEigenlayer,
    pub estimated_next_skimming_slot: u64,
    pub estimated_next_skimming_at: chrono::DateTime<chrono::Utc>,
    pub exit_requested: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Stake {
    pub validator: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumPostStakesRequest {
    pub stakes: Vec<Stake>,
    pub account_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumRewardsRequest {
    pub validators: Vec<String>,
    pub scope: Scope,
    // Addresses
    pub wallets: Vec<String>,
    // Addresses
    pub proxies: Vec<String>,
    pub validator_indexes: Vec<u64>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub include_usd: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumRewardsResponse {
    pub date: chrono::DateTime<chrono::Utc>,
    pub consensus_rewards: String,
    pub execution_rewards: String,
    pub mev_execution_rewards: String,
    pub non_mev_execution_rewards: String,
    pub median_execution_reward: String,
    pub rewards: String,
    pub stake_balance: String,
    pub gross_apy: String,
    pub cl_apy: String,
    pub el_apy: String,
    pub active_validator_count: String,
    pub rewards_usd: String,
    pub stake_balance_usd: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumOperationsRequest {
    // Addresses
    pub validators: Vec<String>,
    // Addresses
    pub wallets: Vec<String>,
    // Addresses
    pub proxies: Vec<String>,
    pub validator_indexes: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

fn deserialize_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let the_type = EthereumOperationsResponse::deserialize(deserializer)?;

    let the_type = match the_type {
        EthereumOperationsResponse::Deposit { .. } => "deposit".into(),
        EthereumOperationsResponse::ConsensusWithdrawal { .. } => "consensus_withdrawal".into(),
        EthereumOperationsResponse::ExecutionReward { .. } => "execution_reward".into(),
        EthereumOperationsResponse::KilnExitRequest { .. } => "kiln_exit_request".into(),
        EthereumOperationsResponse::LidoExitRequest { .. } => "lido_exit_request".into(),
        EthereumOperationsResponse::RioExitRequest { .. } => "rio_exit_request".into(),
        EthereumOperationsResponse::VoluntaryExit { .. } => "voluntary_exit".into(),
    };

    Ok(the_type)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum EthereumOperationsResponse {
    Deposit {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        tx_hash: String,
        tx_sender: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        // Addresses
        proxies: Vec<String>,
        slot: u64,
        block: u64,
        block_base_fee: Option<String>,
        withdrawal_credentials: String,
        amount: String,
    },
    ConsensusWithdrawal {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        slot: u64,
        block: u64,
        fee_recipient: String,
        amount: String,
    },
    ExecutionReward {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        slot: u64,
        block: u64,
        fee_recipient: String,
        is_mev_block: bool,
        mev_payout_tx_hash: String,
        amount: String,
    },
    KilnExitRequest {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        tx_hash: String,
        tx_gas_used: String,
        tx_effective_gas_price: Option<String>,
        // Address
        tx_sender: String,
        slot: u64,
        block: u64,
        block_base_fee: Option<String>,
        // Address
        emitting_contract: String,
        // Address
        caller: Option<String>,
    },
    LidoExitRequest {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        tx_hash: String,
        tx_gas_used: String,
        tx_effective_gas_price: Option<String>,
        // Address
        tx_sender: String,
        slot: u64,
        block: u64,
        block_base_fee: Option<String>,
        // Address
        emitting_contract: String,
    },
    RioExitRequest {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        tx_hash: String,
        tx_gas_used: String,
        tx_effective_gas_price: Option<String>,
        // Address
        tx_sender: String,
        slot: u64,
        block: u64,
        block_base_fee: Option<String>,
        // Address
        emitting_contract: String,
    },
    VoluntaryExit {
        #[serde(deserialize_with = "deserialize_type")]
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        // Address
        validator_address: String,
        validator_index: NonZeroU64,
        slot: u64,
        block: u64,
        index_in_payload: u64,
        message_epoch: u64,
        message_signature: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumNetworkStats {
    pub network_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub eth_price_usd: f64,
    pub estimated_entry_time_seconds: u64,
    pub estimated_exit_time_seconds: u64,
    pub estimated_withdrawal_time_seconds: u64,
    pub nb_validators: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GrossAPY {
    pub last_1d: u64,
    pub last_7d: u64,
    pub last_30d: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KilnStats {
    pub gross_apy: GrossAPY,
}

#[derive(Serialize, Deserialize, Debug, Default, AsRefStr, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum ValidationKeysFormat {
    #[default]
    BatchDeposit,
    CliDeposit,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PostValidationKeysRequest {
    pub account_id: Uuid,
    // Address
    pub withdrawal_address: String,
    // Address
    pub fee_recipient_address: String,
    pub number: u64,
    pub format: ValidationKeysFormat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PostETHStakesTx {
    pub account_id: Uuid,
    pub wallet: String,
    pub amount_wei: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PostETHStakesTxResponse {
    pub unsigned_tx_hash: String,
    pub unsigned_tx_serialized: String,
    pub to: String,
    pub contract_call_data: String,
    pub amount_wei: String,
    pub nonce: u64,
    pub gas_limit: u64,
    pub max_priority_fee_per_gas_wei: String,
    pub max_fee_per_gas_wei: String,
    pub chain_id: ChainID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ValidationKeys {
    BatchResponse {
        format: ValidationKeysFormat,
        // Addresses
        pubkeys: Vec<String>,
        withdrawal_credentials: Vec<String>,
        signatures: Vec<String>,
        deposit_data_roots: Vec<String>,
    },
    CliResponse {
        format: ValidationKeysFormat,
        // Address
        pubkey: String,
        withdrawal_credentials: String,
        amount: u64,
        signature: String,
        deposit_message_root: String,
        deposit_data_root: String,
        fork_version: String,
        network_name: String,
        deposit_cli_version: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SignedMessage {
    pub pubkey: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signed_messages: Vec<SignedMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumPrepareTxResponse {
    pub signed_tx_serialized: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RequestExitTx {
    pub wallet: String,
    pub validators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RequestExitTxResponse {
    pub unsigned_tx_hash: String,
    pub unsigned_tx_serialized: String,
    pub to: String,
    pub contract_call_data: String,
    pub amount_wei: String,
    pub nonce: u64,
    pub gas_limit: u64,
    pub max_priority_fee_per_gas_wei: String,
    pub max_fee_per_gas_wei: String,
    pub chain_id: ChainID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TxDecodingRequest {
    pub tx_serialized: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumTxDecodingResponse {
    pub r: String,
    pub s: String,
    pub v: String,
    pub y_parity: u64,
    pub chain_id: ChainID,
    pub r#type: String,
    pub to: String,
    pub gas: String,
    pub data: String,
    pub nonce: u64,
    pub value: String,
    pub max_fee_per_gas: String,
    pub max_priority_fee_per_gas: String,
    pub function_name: String,
    pub args: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EthereumReportsRequest {
    pub validators: Vec<String>,
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ExitMessageResponse {
    pub validator_address: String,
    pub pgp_public_key: String,
    pub payload: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug)]
pub struct KilnEthereumClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnEthereumClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/eth", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnEthereumClient {
    pub fn get_stakes(
        &self,
        ethereum_stake_request: EthereumStakesRequest,
    ) -> Result<PaginatedData<Vec<EthereumStakesResponse>>, ureq::Error> {
        let url: String = match ethereum_stake_request {
            EthereumStakesRequest::Network(network_stakes_request) => {
                let EthereumNetworkStakesRequest {
                    validators,
                    validator_indexes,
                    accounts,
                    current_page,
                    page_size,
                    filtered_states,
                    include_eigenlayer,
                    proxies,
                    wallets,
                    withdrawal_credentials,
                } = network_stakes_request;

                let validators = validators.join(",");

                let validators_indexes = validator_indexes
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let wallets = wallets.join(",");

                let accounts = accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let filtered_states = filtered_states
                    .iter()
                    .map(|state| state.as_ref())
                    .collect::<Vec<&str>>()
                    .join(",");

                let proxies = proxies.join(",");

                let withdrawal_credentials = withdrawal_credentials.join(",");

                let url = format!(
                    "{}/stakes?validators={}&wallets={}&proxies={}&withdrawal_credentials={}&validator_indexes={}&include_eigenlayer={}&accounts={}&current_page={}&page_size={}&filtered_states={}",
                    self.base_url,
                    validators,
                    wallets,
                    proxies,
                    withdrawal_credentials,
                    validators_indexes,
                    include_eigenlayer,
                    accounts,
                    current_page,
                    page_size,
                    filtered_states,
                );

                url
            }
            EthereumStakesRequest::Scoped(scoped_stakes_request) => {
                let EthereumScopedStakesRequest {
                    validators,
                    scope,
                    validator_indexes,
                    accounts,
                    current_page,
                    page_size,
                    include_eigenlayer,
                    proxies,
                    wallets,
                    withdrawal_credentials,
                } = scoped_stakes_request;

                let validators = validators.join(",");

                let validators_indexes = validator_indexes
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let wallets = wallets.join(",");

                let accounts = accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let proxies = proxies.join(",");

                let withdrawal_credentials = withdrawal_credentials.join(",");

                let url = format!(
                    "{}/stakes?validators={}&scope={}&wallets={}&proxies={}&withdrawal_credentials={}&validator_indexes={}&include_eigenlayer={}&accounts={}&current_page={}&page_size={}",
                    self.base_url,
                    validators,
                    scope.as_ref(),
                    wallets,
                    proxies,
                    withdrawal_credentials,
                    validators_indexes,
                    include_eigenlayer,
                    accounts,
                    current_page,
                    page_size
                );

                url
            }
        };

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<PaginatedData<Vec<EthereumStakesResponse>>>();

        data
    }

    pub fn post_stakes(
        &self,
        ethereum_create_stake_request: &EthereumPostStakesRequest,
    ) -> Result<PaginatedData<Vec<EthereumStakesResponse>>, ureq::Error> {
        let data = ureq::post(&self.base_url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(ethereum_create_stake_request)?
            .body_mut()
            .read_json::<PaginatedData<Vec<EthereumStakesResponse>>>();

        data
    }

    pub fn rewards(
        &self,
        ethereum_rewards_request: &EthereumRewardsRequest,
    ) -> Result<ReturnedData<Vec<EthereumRewardsResponse>>, ureq::Error> {
        let EthereumRewardsRequest {
            validators,
            scope,
            wallets,
            proxies,
            validator_indexes,
            accounts,
            start_date,
            end_date,
            include_usd,
        } = ethereum_rewards_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let proxies = proxies.join(",");

        let validators_indexes = validator_indexes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = format!(
            "{}/rewards?validators={}&scope={}&wallets={}&proxies={}&validators_indexes={}&accounts={}&start_date={}&end_date={}&include_usd={}",
            self.base_url,
            validators,
            scope.as_ref(),
            wallets,
            proxies,
            validators_indexes,
            accounts,
            start_date,
            end_date,
            include_usd
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<EthereumRewardsResponse>>>();

        data
    }

    pub fn get_operations(
        &self,
        ethereum_operations_request: &EthereumOperationsRequest,
    ) -> Result<ReturnedData<Vec<EthereumOperationsResponse>>, ureq::Error> {
        let EthereumOperationsRequest {
            validators,
            wallets,
            proxies,
            validator_indexes,
            accounts,
            start_date,
            end_date,
        } = ethereum_operations_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let proxies = proxies.join(",");

        let validators_indexes = validator_indexes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = format!(
            "{}/operations?validators={}&wallets={}&proxies={}&validators_indexes={}&accounts={}&start_date={}&end_date={}",
            self.base_url,
            validators,
            wallets,
            proxies,
            validators_indexes,
            accounts,
            start_date,
            end_date,
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<EthereumOperationsResponse>>>();

        data
    }

    pub fn network_stats(&self) -> Result<ReturnedData<EthereumNetworkStats>, ureq::Error> {
        let url = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<EthereumNetworkStats>>();

        data
    }

    pub fn kiln_stats(&self) -> Result<ReturnedData<KilnStats>, ureq::Error> {
        let url = format!("{}/kiln-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<KilnStats>>();

        data
    }

    pub fn post_keys(
        &self,
        post_validation_keys_request: &PostValidationKeysRequest,
    ) -> Result<ReturnedData<Vec<ValidationKeys>>, ureq::Error> {
        let url = format!("{}/keys", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(post_validation_keys_request)?
            .body_mut()
            .read_json::<ReturnedData<Vec<ValidationKeys>>>();

        data
    }

    pub fn post_stake_tx(
        &self,
        post_eth_stake_tx: &PostETHStakesTx,
    ) -> Result<ReturnedData<PostETHStakesTxResponse>, ureq::Error> {
        let url = format!("{}/stake", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(post_eth_stake_tx)?
            .body_mut()
            .read_json::<ReturnedData<PostETHStakesTxResponse>>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_tx_request: &EthereumPrepareTxRequest,
    ) -> Result<ReturnedData<EthereumPrepareTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/prepare", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(prepare_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<EthereumPrepareTxResponse>>();

        data
    }

    pub fn post_broadcast_tx(
        &self,
        tx_serialized: &BroadcastTxRequest,
    ) -> Result<ReturnedData<BroadcastTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/broadcast", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(tx_serialized)?
            .body_mut()
            .read_json::<ReturnedData<BroadcastTxResponse>>();

        data
    }

    pub fn get_status_tx(
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

    pub fn post_request_exit_tx(
        &self,
        request_exit_tx: &RequestExitTx,
    ) -> Result<ReturnedData<RequestExitTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/exit-request", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(request_exit_tx)?
            .body_mut()
            .read_json::<ReturnedData<RequestExitTxResponse>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_decoding_request: &TxDecodingRequest,
    ) -> Result<EthereumTxDecodingResponse, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_decoding_request.tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<EthereumTxDecodingResponse>();

        data
    }

    pub fn get_reports(
        &self,
        ethereum_reports_request: &EthereumReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let EthereumReportsRequest {
            validators,
            wallets,
            accounts,
        } = ethereum_reports_request;

        let validators = validators.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/reports?validators={}&wallets={}&accounts={}",
            self.base_url, validators, wallets, accounts
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

    pub fn get_exit_message(
        &self,
        validators: Vec<String>,
    ) -> Result<ReturnedData<Vec<ExitMessageResponse>>, ureq::Error> {
        let validators = validators.join(",");

        let url: String = format!(
            "{}/transaction/exit-messages?validators={}",
            self.base_url, validators
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<ExitMessageResponse>>>();

        data
    }
}
