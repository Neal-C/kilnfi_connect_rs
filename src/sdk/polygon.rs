use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxRequest, BroadcastTxResponse, ChainID, PostStakesRequest, PostStakesResponse,
    PrepareTxResponse, Reward, RewardRequest,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonGetStakesRequest {
    // Addresses
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub validators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonUnboundWithID {
    pub nonce: u64,
    pub balance: String,
    pub epoch: u64,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonStakesResponse {
    pub delegator_address: String,
    pub validator_index: String,
    pub state: String,
    pub delegated_block: u64,
    pub balance: String,
    pub shares: String,
    pub unbounded_balance: String,
    pub unbounded_epoch: u64,
    pub is_unbounding_complete: bool,
    pub rewards: String,
    pub net_apy: f64,
    pub available_rewards: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub unbounds_with_ids: Vec<PolygonUnboundWithID>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonOperationsRequest {
    // Addresses
    pub wallets: Vec<String>,
    pub validator_indexes: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PolygonOperation {
    ShareMinted {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        amount: String,
        tokens: String,
    },
    ShareBurned {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        amount: String,
        tokens: String,
    },
    ShareBurnedWithID {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        amount: String,
        tokens: String,
        nonce: String,
    },
    DelegatorRestaked {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        total_staked: String,
    },
    DelegatorUnstaked {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        amount: String,
    },
    DelegatorUnstakedWithID {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        amount: String,
        nonce: String,
    },
    DelegatorClaimedRewards {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        user: String,
        amount: String,
        rewards: String,
    },
    SharesTransfer {
        r#type: String,
        block: u64,
        time: chrono::DateTime<chrono::Utc>,
        tx_hash: String,
        tx_index: u64,
        tx_sender: String,
        tx_method_name: String,
        tx_gas_used: String,
        tx_effective_gas_price: String,
        tx_cumulative_gas_used: String,
        validator_id: u64,
        from: String,
        to: String,
        value: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonNetworkStatsResponse {
    pub pol_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonReportsRequest {
    pub validator_indexes: Vec<String>,
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonApproveTxRequest {
    // Address
    pub wallet: String,
    pub contract: String,
    pub amount_wei: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonBuyVoucherTxRequest {
    pub account_id: Uuid,
    pub wallet: String,
    pub amount_wei: String,
    pub validator_share_proxy_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonSellVoucherTxRequest {
    pub wallet: String,
    pub amount_wei: String,
    pub validator_share_proxy_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonTxRequest {
    pub wallet: String,
    pub validator_share_proxy_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub r: String,
    pub s: String,
    pub v: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonTxResponse {
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
#[serde(rename_all = "camelCase")]
pub struct PolygonTxStatusReceiptLog {
    pub address: String,
    pub block_hash: String,
    pub block_number: u64,
    pub data: String,
    pub log_index: u64,
    pub removed: bool,
    pub topics: Vec<String>,
    pub transaction_hash: String,
    pub transaction_index: u64,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolygonTxStatusReceipt {
    pub block_hash: String,
    pub block_number: u64,
    pub contract_address: Option<String>,
    pub cumulative_gas_used: u64,
    pub effective_gas_price: u64,
    pub from: String,
    pub gas_used: u64,
    pub logs: Vec<PolygonTxStatusReceiptLog>,
    pub logs_bloom: String,
    pub status: bool,
    pub to: String,
    pub transaction_hash: String,
    pub transaction_index: u64,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, EnumString, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum PolygonTxStatus {
    #[strum(serialize = "success")]
    Success,
    #[strum(serialize = "error")]
    Error,
    #[strum(serialize = "pending_confirmation")]
    PendingConfirmation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolygonTxStatusResponse {
    pub status: PolygonTxStatus,
    pub receipt: PolygonTxStatusReceipt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolygonTxDecodingResponse {
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

#[derive(Clone, Debug)]
pub struct KilnPolygonClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnPolygonClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/pol", kiln.base_url);

        let bearer_token = format!("Bearer {}", kiln.api_token);

        Self {
            base_url: url,
            bearer_token,
        }
    }
}

impl KilnPolygonClient {
    pub fn get_stakes(
        &self,
        stakes_request: &PolygonGetStakesRequest,
    ) -> Result<ReturnedData<Vec<PolygonStakesResponse>>, ureq::Error> {
        let PolygonGetStakesRequest {
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
            .read_json::<ReturnedData<Vec<PolygonStakesResponse>>>();

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
        reward_request: &RewardRequest,
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

    pub fn get_operations(
        &self,
        operations_request: &PolygonOperationsRequest,
    ) -> Result<ReturnedData<Vec<PolygonOperation>>, ureq::Error> {
        let PolygonOperationsRequest {
            wallets,
            validator_indexes,
            accounts,
            start_date,
            end_date,
        } = operations_request;

        let wallets = wallets.join(",");

        let validator_indexes = validator_indexes.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/operations?wallets={}&validator_indexes={}&accounts={}&start_date={}&end_date={}",
            self.base_url, wallets, validator_indexes, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<PolygonOperation>>>();

        data
    }

    pub fn get_network_stats(
        &self,
    ) -> Result<ReturnedData<PolygonNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<PolygonNetworkStatsResponse>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: &PolygonReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let PolygonReportsRequest {
            validator_indexes,
            wallets,
            accounts,
        } = reports_request;

        let validator_indexes = validator_indexes.join(",");

        let wallets = wallets.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/reports?validator_indexes={}&wallets={}&accounts={}",
            self.base_url, validator_indexes, wallets, accounts
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

    pub fn post_approve_tx(
        &self,
        approve_tx_request: &PolygonApproveTxRequest,
    ) -> Result<ReturnedData<PolygonTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/approve", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(approve_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxResponse>>();

        data
    }

    pub fn post_buy_voucher_tx(
        &self,
        buy_voucher_tx_request: &PolygonBuyVoucherTxRequest,
    ) -> Result<ReturnedData<PolygonTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/buy-voucher", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(buy_voucher_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxResponse>>();

        data
    }

    pub fn post_sell_voucher_tx(
        &self,
        sell_voucher_tx_request: &PolygonSellVoucherTxRequest,
    ) -> Result<ReturnedData<PolygonTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/sell-voucher", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(sell_voucher_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxResponse>>();

        data
    }

    pub fn post_unstake_claim_tokens_tx(
        &self,
        unstake_claim_tokens_tx_request: &PolygonTxRequest,
    ) -> Result<ReturnedData<PolygonTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/unstake-claim_tokens", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unstake_claim_tokens_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxResponse>>();

        data
    }

    pub fn post_withdraw_rewards_tx(
        &self,
        withdraw_rewards_tx_request: &PolygonTxRequest,
    ) -> Result<ReturnedData<PolygonTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/withdraw-rewards", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_rewards_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxResponse>>();

        data
    }

    pub fn post_restake_rewards_tx(
        &self,
        restake_tx_request: &PolygonTxRequest,
    ) -> Result<ReturnedData<PolygonTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/restake-rewards", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(restake_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxResponse>>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &PolygonPrepareTxRequest,
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

    pub fn transaction_status(
        &self,
        tx_hash: &str,
    ) -> Result<ReturnedData<PolygonTxStatusResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/status?tx_hash={}", self.base_url, tx_hash);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxStatusResponse>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<ReturnedData<PolygonTxDecodingResponse>, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<PolygonTxDecodingResponse>>();

        data
    }
}
