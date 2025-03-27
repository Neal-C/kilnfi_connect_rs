use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use uuid::Uuid;

use crate::{response_format::ReturnedData, sdk::RewardRequest, Kiln};

use super::{
    ChainStakes, CreateStakeRequest, OperationsResponse, PostStakesResponse, ResponseFormat,
    Reward, StakeOperationsRequest,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoBroadcastTxResponse {
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoCert {
    #[serde(rename = "PascalCase")]
    stake_delegation: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoNetworkStats {
    pub ada_price_usd: f64,
    pub nb_validators: u64,
    pub network_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signed_messages: Vec<SignedMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoPrepareTxResponse {
    pub signed_tx_serialized: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoReportsRequest {
    pub stake_addresses: Vec<String>,
    pub wallets: Vec<String>,
    pub accounts: Vec<String>,
    pub format: ResponseFormat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoStakeCredentials {
    #[serde(rename = "PascalCase")]
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoStakeDelegation {
    pub stake_credential: CardanoStakeCredentials,
    pub pool_keyhash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoStakeTxResponse {
    pub unsigned_tx_hash: String,
    pub unsigned_tx_serialiazed: String,
    pub inputs: Vec<CardanoStakeTxInput>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoStakeTxOutput {
    pub address: String,
    pub amount: CardanoTxAmount,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoStakeTxRequest {
    pub account_id: Uuid,
    pub wallet: String,
    pub pool_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoStakeTxInput {
    pub transaction_id: String,
    pub index: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SignedMessage {
    pub pubkey: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoTxAmount {
    pub coin: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoTxDecodeResponse {
    pub body: CardanoTxDecodingBody,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoTxDecodingBody {
    pub inputs: Vec<CardanoStakeTxInput>,
    pub outputs: Vec<CardanoStakeTxOutput>,
    pub fee: String,
    pub ttl: String,
    pub certs: Vec<CardanoCert>,
    pub witness_cert: serde_json::Value,
    pub is_valid: bool,
}

#[derive(Serialize, Deserialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CardanoTxStatus {
    Success,
    PendingConfirmation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoTxStatusOutputAmount {
    pub unit: String,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoTxStatusReceipt {
    pub hash: String,
    pub block: String,
    pub block_height: u64,
    pub block_time: u64,
    pub slot: u64,
    pub index: u64,
    pub output_amount: Vec<CardanoTxStatusOutputAmount>,
    pub fees: String,
    pub deposit: String,
    pub size: u64,
    pub invalid_before: Option<String>,
    pub invalid_hereafter: Option<String>,
    pub utxo_count: u64,
    pub withdrawal_count: u64,
    pub mir_cert_count: u64,
    pub delegation_count: u64,
    pub stake_cert_count: u64,
    pub pool_update_count: u64,
    pub pool_retire_count: u64,
    pub asset_mint_or_burn_count: u64,
    pub redeemer_count: u64,
    pub valid_contract: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoTxStatusResponse {
    pub status: CardanoTxStatus,
    pub receipt: CardanoTxStatusReceipt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CardanoWithdrawRewardsTxRequest {
    pub wallet: String,
    pub amount_lovelace: Option<String>,
}

#[derive(Clone, Debug)]
pub struct KilnCardanoClient {
    bearer_token: String,
    base_url: String,
}

impl From<&Kiln> for KilnCardanoClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/ada", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnCardanoClient {
    pub fn get_stakes(
        &self,
        wallets: Vec<String>,
        vaults: Vec<String>,
        pool_ids: Vec<String>,
        accounts: Vec<Uuid>,
        current_page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ReturnedData<Vec<ChainStakes>>, ureq::Error> {
        let wallets = wallets.join(",");

        let vaults = vaults.join(",");

        let pool_ids = pool_ids.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let current_page = current_page.unwrap_or(1).to_string();

        let page_size = page_size.unwrap_or(1).to_string();

        let url: String = format!(
            "{}/stakes?wallets={}&vaults={}&pool_ids={}&accounts={}&page_size={}&current_page={}",
            self.base_url, wallets, vaults, pool_ids, accounts, page_size, current_page
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<ChainStakes>>>();

        data
    }

    pub fn get_network_stats(
        &self,
        reports_request: &CardanoReportsRequest,
    ) -> Result<ReturnedData<CardanoNetworkStats>, ureq::Error> {
        let CardanoReportsRequest {
            stake_addresses,
            wallets,
            accounts,
            format,
        } = reports_request;

        let stake_addresses = stake_addresses.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts.join(",");

        let url: String = format!(
            "{}/network-stats?stake_addresses={}&wallets={}&accounts={}&format={}",
            self.base_url,
            stake_addresses,
            wallets,
            accounts,
            format.as_ref()
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<CardanoNetworkStats>>();

        data
    }

    pub fn get_operations(
        &self,
        operation_request: StakeOperationsRequest,
    ) -> Result<ReturnedData<Vec<OperationsResponse>>, ureq::Error> {
        let StakeOperationsRequest {
            stake_addresses,
            wallets,
            pool_ids,
            accounts,
            start_date,
            end_date,
        } = operation_request;

        let stake_addresses = stake_addresses.join(",");

        let wallets = wallets.join(",");

        let pool_ids = pool_ids.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/operations?stake_addresses={}&wallets={}&pool_ids={}&accounts={}&start_date={}&end_date={}",self.base_url, stake_addresses, wallets,pool_ids, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<OperationsResponse>>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: CardanoReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let CardanoReportsRequest {
            stake_addresses,
            wallets,
            accounts,
            format,
        } = reports_request;

        let stake_addresses = stake_addresses.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts.join(",");

        let url: String = format!(
            "{}/reports?stake_addresses={}&wallets={}&accounts={}&format={}",
            self.base_url,
            stake_addresses,
            wallets,
            accounts,
            format.as_ref()
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

    pub fn get_rewards(
        &self,
        reward_request: RewardRequest,
    ) -> Result<ReturnedData<Vec<Reward>>, ureq::Error> {
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
            .read_json::<ReturnedData<Vec<Reward>>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<CardanoTxDecodeResponse, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<CardanoTxDecodeResponse>();

        data
    }

    pub fn post_stakes(
        &self,
        create_stake_request: &CreateStakeRequest,
    ) -> Result<ReturnedData<Vec<PostStakesResponse>>, ureq::Error> {
        let url: String = format!("{}/stakes", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(create_stake_request)?
            .body_mut()
            .read_json::<ReturnedData<Vec<PostStakesResponse>>>();

        data
    }

    pub fn post_stake_tx(
        &self,
        cardano_stake_tx_request: &CardanoStakeTxRequest,
    ) -> Result<ReturnedData<CardanoStakeTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/stake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(cardano_stake_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<CardanoStakeTxResponse>>();

        data
    }

    pub fn post_withdraw_rewards_tx(
        &self,
        withdraw_rewards_tx_request: &CardanoWithdrawRewardsTxRequest,
    ) -> Result<ReturnedData<CardanoStakeTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/withdraw-rewards", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_rewards_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<CardanoStakeTxResponse>>();

        data
    }

    pub fn post_unstake_tx(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<CardanoStakeTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/unstake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(wallet)?
            .body_mut()
            .read_json::<ReturnedData<CardanoStakeTxResponse>>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        cardano_prepare_tx_request: &CardanoPrepareTxRequest,
    ) -> Result<ReturnedData<CardanoPrepareTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/prepare", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(cardano_prepare_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<CardanoPrepareTxResponse>>();

        data
    }

    pub fn post_broadcast_tx(
        &self,
        tx_serialized: &str,
    ) -> Result<ReturnedData<CardanoBroadcastTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/broadcast", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(tx_serialized)?
            .body_mut()
            .read_json::<ReturnedData<CardanoBroadcastTxResponse>>();

        data
    }

    pub fn get_tx_status(
        &self,
        tx_hash: &str,
    ) -> Result<ReturnedData<CardanoTxStatusResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/status?tx_hash={}", self.base_url, tx_hash);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<CardanoTxStatusResponse>>();

        data
    }
}
