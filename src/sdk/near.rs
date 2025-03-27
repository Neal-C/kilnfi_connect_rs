use std::io::Read;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxResponse, PostStakesRequest, PostStakesResponse, PrepareTxResponse, ResponseFormat,
    Reward, RewardRequest,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearGetStakesRequest {
    pub stake_accounts: Vec<Uuid>,
    pub validators: Vec<String>,
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearStakesResponse {
    pub stake_account: String,
    // NEAR account ID
    pub account: String,
    pub validator: String,
    pub balance: String,
    pub rewards: String,
    pub unstaked_balance: String,
    pub can_withdraw: bool,
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub activated_epoch: u64,
    pub activated_block: u64,
    pub net_apy: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearOperationsRequest {
    pub stake_accounts: Vec<Uuid>,
    pub wallets: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearOperationsResponse {
    pub receipt_id: String,
    pub r#type: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub block: u64,
    pub tx_hash: String,
    pub tx_fees: String,
    pub validator: String,
    pub account: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearReportsRequest {
    pub stake_accounts: Vec<Uuid>,
    pub accounts: Vec<Uuid>,
    pub wallets: Vec<String>,
    pub format: ResponseFormat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearNetworkStatsResponse {
    pub near_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // camelCase
pub struct NearEd25519Key {
    pub key_type: u64,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // camelCase
pub struct NearPublicKey {
    pub ed_25519_key: NearEd25519Key,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // it's camelCase for some reason
pub struct NearTx {
    pub signer_id: String,
    #[serde(alias = "publicKey")]
    pub public_key: NearPublicKey,
    pub r#enum: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NearFunctionCallArgs {
    pub r#type: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NearFunctionCall {
    pub method_name: String,
    pub args: NearFunctionCallArgs,
    pub gas: String,
    pub deposit: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NearAction {
    pub function_call: NearFunctionCall,
    pub r#enum: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearTxResponse {
    pub unsigned_tx_serialized: String,
    pub unsigned_tx_hash: String,
    pub tx: NearTx,
    pub nonce: String,
    #[serde(rename(serialize = "receiverId", deserialize = "receiverId"))]
    pub receiver_id: String,
    pub actions: Vec<NearAction>,
    #[serde(rename(serialize = "blockHash", deserialize = "blockHash"))]
    pub block_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearStakeTxRequest {
    pub account_id: Uuid,
    pub wallet: String,
    pub pool_id: String,
    pub amount_yocto: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearUnstakeTxRequest {
    pub wallet: String,
    pub pool_id: String,
    // if omitted with Option::None, all is unstaked
    pub amount_yocto: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearWithdrawRewardsTxRequest {
    pub wallet: String,
    pub pool_id: String,
    // if omitted with Option::None, all is withdrawn
    pub amount_yocto: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearTxDecodingPublicKey {
    #[serde(rename(serialize = "KeyType", deserialize = "KeyType"))]
    pub key_type: u64,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NearTxDecodingResponse {
    pub public_key: NearTxDecodingPublicKey,
    pub signer_id: String,
    pub nonce: String,
    pub receiver_id: String,
    pub actions: Vec<NearAction>,
    pub block_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NearBroadcastTxRequest {
    pub signed_tx_serialized: String,
}

#[derive(Clone, Debug)]
pub struct KilnNearClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnNearClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/near", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnNearClient {
    pub fn get_stakes(
        &self,
        stakes_request: &NearGetStakesRequest,
    ) -> Result<ReturnedData<Vec<NearStakesResponse>>, ureq::Error> {
        let NearGetStakesRequest {
            stake_accounts,
            accounts,
            validators,
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
            .read_json::<ReturnedData<Vec<NearStakesResponse>>>();

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
        operations_request: &NearOperationsRequest,
    ) -> Result<ReturnedData<Vec<NearOperationsResponse>>, ureq::Error> {
        let NearOperationsRequest {
            stake_accounts,
            accounts,
            wallets,
            start_date,
            end_date,
        } = operations_request;

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

        let url = format!(
            "{}/operations?stake_accounts={}&accounts={}&wallets={}&start_date={}&end_date={}",
            self.base_url, stake_accounts, accounts, wallets, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<NearOperationsResponse>>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: &NearReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let NearReportsRequest {
            stake_accounts,
            accounts,
            wallets,
            format,
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
            "{}/reports?stake_accounts={}&accounts={}&wallets={}&format={}",
            self.base_url,
            stake_accounts,
            accounts,
            wallets,
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

    pub fn get_network_stats(&self) -> Result<ReturnedData<NearNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<NearNetworkStatsResponse>>();

        data
    }

    pub fn post_stake_tx(
        &self,
        transaction_stake_request: &NearStakeTxRequest,
    ) -> Result<ReturnedData<NearTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/stake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(transaction_stake_request)?
            .body_mut()
            .read_json::<ReturnedData<NearTxResponse>>();

        data
    }

    pub fn post_unstake_tx(
        &self,
        unstake_transaction_request: &NearUnstakeTxRequest,
    ) -> Result<ReturnedData<NearTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/unstake", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unstake_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<NearTxResponse>>();
        data
    }

    pub fn post_withdraw_rewards_tx(
        &self,
        withdraw_rewards_transaction_request: &NearWithdrawRewardsTxRequest,
    ) -> Result<ReturnedData<NearTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/withdraw-rewards", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_rewards_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<NearTxResponse>>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &NearPrepareTxRequest,
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
        broadcast_transaction_request: &NearBroadcastTxRequest,
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

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<NearTxDecodingResponse, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<NearTxDecodingResponse>();

        data
    }
}
