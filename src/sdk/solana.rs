use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxRequest, BroadcastTxResponse, PostStakesResponse, PrepareTxResponse, ResponseFormat,
    StakeState,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaGetStakesRequest {
    pub stake_accounts: Vec<Uuid>,
    pub validators: Vec<String>,
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaStakesResponse {
    pub stake_account: String,
    pub vote_account: String,
    pub withdraw_pubkey: String,
    pub state: StakeState,
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub activated_epoch: u64,
    pub deactivated_at: chrono::DateTime<chrono::Utc>,
    pub deactivated_epoch: u64,
    pub balance: String,
    pub rewards: String,
    pub net_apy: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaStakeRequestStake {
    pub stake_account: String,
    pub balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaPostStakesRequest {
    pub stakes: Vec<SolanaStakeRequestStake>,
    pub account_id: Uuid,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SolanaRewardRequest {
    Daily {
        stakes_accounts: Vec<String>,
        validators: Vec<String>,
        wallets: Vec<String>,
        accounts: Vec<Uuid>,
        format: ResponseFormat,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        include_usd: bool,
    },
    Epoch {
        stakes_addresses: Vec<String>,
        wallets: Vec<String>,
        pool_ids: Vec<String>,
        accounts: Vec<Uuid>,
        format: ResponseFormat,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        start_epoch: u64,
        end_epoch: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SolanaReward {
    Epoch {
        epoch: u64,
        epoch_ts: chrono::DateTime<chrono::Utc>,
        rewards: String,
        gross_mev_rewards: String,
        mev_comission: String,
        active_balance: String,
        net_apy: f64,
    },
    Daily {
        date: chrono::DateTime<chrono::Utc>,
        rewards: String,
        gross_mev_rewards: String,
        mev_comission: String,
        active_balance: String,
        net_apy: f64,
        rewards_usd: String,
        rewards_balance_usd: Option<u64>,
        active_balance_usd: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaOperationsRequest {
    pub stake_accounts: Vec<Uuid>,
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SolanaOperation {
    CreateAccountWithSeed {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Address
        stake_account: String,
        amount: String,
        // Address
        stake_authority: String,
    },
    CreateAccount {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: String,
        amount: String,
        // Solana Address
        stake_authority: String,
    },
    Delegate {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: String,
        // Solana Address
        vote_account: String,
        stake_authority: String,
    },
    Deactivate {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: String,
        // Solana Address
        stake_authority: String,
    },
    Redelegate {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: String,
        // Solana Address
        new_stake_account: String,
        // Solana Address
        vote_account: String,
        // Solana Address
        stake_authority: String,
    },
    Split {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: Option<String>,
        // Solana Address
        new_stake_account: String,
        amount: String,
        // Solana Address
        stake_authority: String,
    },
    Withdraw {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: String,
        // Solana Address
        destination_account: String,
        amount: String,
        // Solana Address
        stake_authority: String,
    },
    Merge {
        r#type: String,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_fee: String,
        tx_memo: Option<String>,
        block: u64,
        // Solana Address
        stake_account: String,
        // Solana Address
        source_stake_account: String,
        // Solana Address
        stake_authority: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaNetworkStatsResponse {
    pub sol_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaReportsRequest {
    pub stake_accounts: Vec<Uuid>,
    pub accounts: Vec<Uuid>,
    pub wallets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NonceAccount {
    // Solana Address
    pub nonce_account: String,
    // Solana Address
    pub nonce_account_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaUnsignedTxHeader {
    pub num_required_signatures: u64,
    pub num_readonly_signed_accounts: u64,
    pub num_readonly_unsigned_accounts: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaUnsignedTxInstruction {
    pub program_id_index: u64,
    pub accounts: Vec<u64>,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaUnsignedTx {
    pub header: SolanaUnsignedTxHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<SolanaUnsignedTxInstruction>,
    pub index_to_program_ids: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaTxResponse {
    pub unsigned_tx_hash: String,
    pub unsigned_tx_serialized: String,
    pub unsigned_tx: SolanaUnsignedTx,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaStakeTxRequest {
    pub account_id: Uuid,
    // Solana Address
    pub wallet: String,
    pub amount_lamports: String,
    // Solana Address
    pub vote_account_address: String,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaDeactivateStakeTxRequest {
    // Solana Address
    pub stake_account: String,
    pub wallet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaWithdrawStakeTxRequest {
    // Solana Address
    pub stake_account: String,
    // Solana Address
    pub wallet: String,
    pub amount_lamports: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaMergeStakeTxRequest {
    // Solana Address
    pub stake_account_source: String,
    // Solana Address
    pub stake_account_destination: String,
    // Solana Address
    pub wallet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaSplitStakeTxRequest {
    pub account_id: Uuid,
    // Solana Address
    pub stake_account: String,
    // Solana Address
    pub wallet: String,
    pub amount_lamports: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTokenAmount {
    pub amount: String,
    pub decimals: String,
    pub ui_amount: Option<u64>,
    pub ui_amount_string: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTokenBalance {
    pub account_index: u64,
    pub mint: String,
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LoadedAddress {
    // Addresses
    pub writable: Vec<String>,
    // Addresses
    pub readonly: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxReceiptMeta {
    pub compute_units_consumed: u64,
    pub err: Option<String>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<SolanaTokenBalance>>,
    pub post_token_balances: Option<Vec<SolanaTokenBalance>>,
    pub loaded_addresses: Vec<LoadedAddress>,
    // Unclear what type it should be, so I'll just leave it like that until it blows up
    pub rewards: Vec<SolanaTxReceiptReward>,
    // Unclear what type it should be, so I'll just leave it like that until it blows up
    pub status: SolanaTxReceiptStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxReceiptReward {
    pub pubkey: String,
    pub lamports: u64,
    pub post_balance: Option<u64>,
    pub reward_type: Option<u64>,
    pub commission: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SolanaTxReceiptStatus {
    pub ok: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SolanaTxCore {
    pub message: SolanaUnsignedTx,
    pub signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxReceipt {
    pub slot: u64,
    pub blocktime: Option<u64>,
    pub meta: Option<SolanaTxReceiptMeta>,
    pub transaction: SolanaTxCore,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SolanaTxStatusResponse {
    pub status: String,
    pub receipt: SolanaTxReceipt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxDecodeInstructionKey {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxDecodeInstruction {
    pub keys: Vec<SolanaTxDecodeInstructionKey>,
    // Solana Address
    pub program_id: String,
    pub data: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaTxDecodeResponse {
    pub recent_blockhash: String,
    pub fee_payer: String,
    // Kept like this until it blows up
    pub nonce_info: Option<String>,
    pub instructions: Vec<SolanaTxDecodeInstruction>,
    pub signers: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct KilnSolanaClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnSolanaClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/sol", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnSolanaClient {
    pub fn get_stakes(
        &self,
        stakes_request: &SolanaGetStakesRequest,
    ) -> Result<ReturnedData<Vec<SolanaStakesResponse>>, ureq::Error> {
        let SolanaGetStakesRequest {
            validators,
            stake_accounts,
            accounts,
            wallets,
        } = stakes_request;

        let validators = validators.join(",");

        let stake_accounts = stake_accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let wallets = wallets.join(",");

        let url: String = format!(
            "{}/stakes?validators={}&stake_accounts={}&accounts={}&wallets={}",
            self.base_url, validators, stake_accounts, accounts, wallets
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<SolanaStakesResponse>>>();

        data
    }

    pub fn post_stakes(
        &self,
        post_stakes_request: &SolanaPostStakesRequest,
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
        reward_request: SolanaRewardRequest,
    ) -> Result<ReturnedData<Vec<SolanaReward>>, ureq::Error> {
        let url: String = match reward_request {
            SolanaRewardRequest::Daily {
                stakes_accounts,
                validators,
                wallets,
                accounts,
                format,
                start_date,
                end_date,
                include_usd,
            } => {
                let stakes_accounts = stakes_accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let validators = validators.join(",");

                let wallets = wallets.join(",");

                let accounts = accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let url = format!("{}/rewards?stake_accounts={}&validators={}&wallets={}&accounts={}&format={}&start_date={}&end_date={}&include_usd={}", self.base_url, stakes_accounts, validators, wallets, accounts, format.as_ref(), start_date, end_date, include_usd);

                url
            }
            SolanaRewardRequest::Epoch {
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

                let wallets = wallets.join(",");

                let pool_ids = pool_ids.join(",");

                let accounts = accounts
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                let url = format!("{}/rewards?stakes_addresses={}&wallets={}&pool_ids={}&accounts={}&format={}&start_date={}&end_date={}&start_epoch={}&end_epoch={}", self.base_url, stakes_addresses, wallets, pool_ids, accounts, format.as_ref(), start_date, end_date, start_epoch, end_epoch);

                url
            }
        };

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<SolanaReward>>>();

        data
    }

    pub fn get_operations(
        &self,
        validator_operations_request: &SolanaOperationsRequest,
    ) -> Result<ReturnedData<Vec<SolanaOperation>>, ureq::Error> {
        let SolanaOperationsRequest {
            stake_accounts,
            wallets,
            accounts,
            start_date,
            end_date,
        } = validator_operations_request;

        let stake_accounts = stake_accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/operations?stake_accounts={}&wallets={}&accounts={}&start_date={}&end_date={}",
            self.base_url, stake_accounts, wallets, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<SolanaOperation>>>();

        data
    }

    pub fn get_network_stats(
        &self,
    ) -> Result<ReturnedData<SolanaNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<SolanaNetworkStatsResponse>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: &SolanaReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let SolanaReportsRequest {
            stake_accounts,
            accounts,
            wallets,
        } = reports_request;

        let stake_accounts = stake_accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let wallets = wallets.join(",");

        let url: String = format!(
            "{}/reports?stake_accounts={}&accounts={}&wallets={}",
            self.base_url, stake_accounts, accounts, wallets,
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

    pub fn get_nonce_account(&self) -> Result<ReturnedData<NonceAccount>, ureq::Error> {
        let url = format!("{}/nonce-account", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<NonceAccount>>();

        data
    }

    pub fn post_stake_tx(
        &self,
        transaction_stake_request: &SolanaStakeTxRequest,
    ) -> Result<ReturnedData<SolanaTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/stake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(transaction_stake_request)?
            .body_mut()
            .read_json::<ReturnedData<SolanaTxResponse>>();

        data
    }

    pub fn post_deactivate_stake_tx(
        &self,
        deactivate_stake_tx: &SolanaDeactivateStakeTxRequest,
    ) -> Result<ReturnedData<SolanaTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/deactivate-stake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(deactivate_stake_tx)?
            .body_mut()
            .read_json::<ReturnedData<SolanaTxResponse>>();

        data
    }

    pub fn post_withdraw_stake_tx(
        &self,
        withdraw_stake_tx: &SolanaWithdrawStakeTxRequest,
    ) -> Result<ReturnedData<SolanaTxResponse>, ureq::Error> {
        let url = format!("{}/withdraw-stake", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_stake_tx)?
            .body_mut()
            .read_json::<ReturnedData<SolanaTxResponse>>();

        data
    }

    pub fn post_merge_stake_tx(
        &self,
        merge_stake_tx: &SolanaMergeStakeTxRequest,
    ) -> Result<ReturnedData<SolanaTxResponse>, ureq::Error> {
        let url = format!("{}/merge-stake", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(merge_stake_tx)?
            .body_mut()
            .read_json::<ReturnedData<SolanaTxResponse>>();

        data
    }

    pub fn post_split_stake_tx(
        &self,
        split_stake_tx: &SolanaSplitStakeTxRequest,
    ) -> Result<ReturnedData<SolanaTxResponse>, ureq::Error> {
        let url = format!("{}/split-stake", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(split_stake_tx)?
            .body_mut()
            .read_json::<ReturnedData<SolanaTxResponse>>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &SolanaPrepareTxRequest,
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

    pub fn get_tx_status(&self, tx_hash: &str) -> Result<SolanaTxStatusResponse, ureq::Error> {
        let url: String = format!("{}/transaction/status?tx_hash={}", self.base_url, tx_hash);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<SolanaTxStatusResponse>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<ReturnedData<SolanaTxDecodeResponse>, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<SolanaTxDecodeResponse>>();

        data
    }
}
