use std::io::Read;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxRequest, BroadcastTxResponse, GetStakesRequest, GetStakesResponse,
    PostStakesRequest, PostStakesResponse, PrepareTxRequest, PrepareTxResponse, ReportsRequest,
    RestakeRewardsTxRequest, Reward, RewardRequest, StakingOperation, TxDecodingResponse,
    TxResponse, TxStatusResponse, ValidatorOperationsRequest, WithdrawRewardsTxRequest,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CosmosNetworkStatsResponse {
    pub atom_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CosmosStakeTxRequest {
    pub account_id: Uuid,
    pub pubkey: String,
    pub validator: String,
    pub amount_uatom: String,
    pub restake_rewards: bool,
    pub grantee_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CosmosUnstakeRewardsTxRequest {
    // Pubkey
    pub pubkey: String,
    // Address
    pub validator: String,
    pub amount_uatom: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CosmosRedelegateTxRequest {
    pub account_id: Uuid,
    // Pubkey
    pub pubkey: String,
    pub validator_source: String,
    pub validator_destination: String,
    pub amount_uatom: String,
}

#[derive(Clone, Debug)]
pub struct KilnCosmosClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnCosmosClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/atom", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnCosmosClient {
    pub fn get_stakes(
        &self,
        cosmos_stake_request: &GetStakesRequest,
    ) -> Result<ReturnedData<Vec<GetStakesResponse>>, ureq::Error> {
        let GetStakesRequest {
            validators,
            delegators,
            accounts,
        } = cosmos_stake_request;

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
            .read_json::<ReturnedData<Vec<GetStakesResponse>>>();

        data
    }

    pub fn post_stakes(
        &self,
        cosmos_create_stake_request: &PostStakesRequest,
    ) -> Result<ReturnedData<PostStakesResponse>, ureq::Error> {
        let url = format!("{}/stakes", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(cosmos_create_stake_request)?
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
        validator_operations_request: &ValidatorOperationsRequest,
    ) -> Result<ReturnedData<Vec<StakingOperation>>, ureq::Error> {
        let ValidatorOperationsRequest {
            validators,
            delegators,
            accounts,
            authz,
            start_date,
            end_date,
        } = validator_operations_request;

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
            .read_json::<ReturnedData<Vec<StakingOperation>>>();

        data
    }

    pub fn get_network_stats(
        &self,
    ) -> Result<ReturnedData<CosmosNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<CosmosNetworkStatsResponse>>();

        data
    }

    pub fn get_reports(&self, reports_request: &ReportsRequest) -> Result<Vec<u8>, ureq::Error> {
        let ReportsRequest {
            validators,
            delegators,
            accounts,
        } = reports_request;

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

    pub fn post_stake_tx(
        &self,
        transaction_stake_request: &CosmosStakeTxRequest,
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

    pub fn post_restake_rewards_tx(
        &self,
        restake_rewards_transaction_request: &RestakeRewardsTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/restake-rewards", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(restake_rewards_transaction_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();

        data
    }

    pub fn unstake_rewards_transaction(
        &self,
        unstake_rewards_transaction_request: &CosmosUnstakeRewardsTxRequest,
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
        cosmos_redelegate_transaction_request: &CosmosRedelegateTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/redelegate", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(cosmos_redelegate_transaction_request)?
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
