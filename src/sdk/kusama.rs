use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{BroadcastTxRequest, BroadcastTxResponse, PrepareTxResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaGetStakesRequest {
    pub addresses: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaStakesResponse {
    // Address
    pub address: String,
    pub pool_id: u64,
    pub active_balance: String,
    pub unbonding_balance: String,
    pub withdrawable_balance: String,
    pub net_rewards: String,
    pub gross_rewards: String,
    pub withdrawable_rewards: String,
    pub state: String,
    pub net_apy: f64,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub undelegated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KusamaReward {
    ByDay {
        net_rewards: String,
        gross_rewards: String,
        date: chrono::DateTime<chrono::Utc>,
        active_balance: String,
        net_apy: f64,
        gross_apy: f64,
        active_balance_usd: u64,
        net_rewards_usd: u64,
        gross_rewards_usd: u64,
    },
    ByEra {
        net_rewards: String,
        gross_rewards: String,
        era: u64,
        active_balance: String,
        net_apy: f64,
        active_balance_usd: u64,
        net_rewards_usd: u64,
        gross_rewards_usd: u64,
    },
}

#[derive(Serialize, Deserialize, AsRefStr, Default, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum KusamaResponseFormat {
    #[default]
    Daily,
    Era,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaRewardRequest {
    pub addresses: Vec<String>,
    pub pool_ids: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub format: KusamaResponseFormat,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaOperation {
    pub r#type: String,
    pub block_number: u64,
    pub block_time: chrono::DateTime<chrono::Utc>,
    pub tx_hash: String,
    pub extrinsic_seq_id: u64,
    pub event_seq_id: u64,
    pub amount: String,
    // Address
    pub address: String,
    pub validator: String,
    pub pool_id: String,
    pub earned_era: String,
    pub era: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaOperationsRequest {
    pub addresses: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaReportsRequest {
    pub addresses: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaNetworkStatsResponse {
    pub ksm_price_usd: f64,
    pub nb_validators: u64,
    pub net_gross_apy: f64,
    pub supply_staked_percent: f64,
    pub inflation_rate: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payee {
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KusamaTxMethodArgs {
    pub value: String,
    pub payee: Payee,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KusamaTxMethod {
    pub args: KusamaTxMethodArgs,
    pub name: String,
    pub pallet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KusamaUnsignedTx {
    pub block_hash: String,
    pub era_period: u64,
    pub genesis_hash: String,
    pub metadata_rpc: String,
    pub method: KusamaTxMethod,
    pub nonce: u64,
    pub spec_version: u64,
    pub tip: u64,
    pub transaction_version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxResponse {
    pub unsigned_tx_payload: String,
    pub unsigned_tx_serialized: String,
    pub unsigned_tx: KusamaUnsignedTx,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaBondTxRequest {
    pub account_id: Uuid,
    // Address
    pub stash_account: String,
    pub amount_planck: String,
    pub reward_destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaBondExtraTxRequest {
    // Address
    pub stash_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaRebondTxRequest {
    // Address
    pub stash_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaUnbondTxRequest {
    // Address
    pub stash_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaNominateTxRequest {
    // Address
    pub stash_account: String,
    pub validator_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaWithdrawUnbondedTxRequest {
    // Address
    pub stash_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaChillTxRequest {
    // Address
    pub stash_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaSetPayeeTxRequest {
    // Address
    pub stash_account: String,
    // Address
    pub reward_destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaJoinPoolTxRequest {
    pub account_id: Uuid,
    pub member_account: String,
    pub amount_planck: String,
    pub pool_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaBondExtraPoolTxRequest {
    pub member_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaPoolTxRequest {
    pub member_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaUnbondFromPoolTxRequest {
    pub member_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponseDataParam {
    pub name: String,
    pub r#type: String,
    pub type_name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponseDataEvent {
    pub event_index: String,
    pub block_num: u64,
    pub extrinsic_idx: u64,
    pub module_id: String,
    pub event_id: String,
    pub params: String,
    pub phase: u64,
    pub event_idx: u64,
    pub extrinsic_hash: String,
    pub finalized: bool,
    pub block_timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponseDataError {
    pub module: String,
    pub name: String,
    pub doc: String,
    pub value: String,
    pub batch_index: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponseDataLifetime {
    pub birth: u64,
    pub death: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponseDataAccountDisplay {
    // Address
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponseData {
    pub block_timestamp: u64,
    pub block_num: u64,
    pub extrinsic_index: String,
    pub call_module_function: String,
    pub call_module: String,
    pub account_id: String,
    pub signature: String,
    pub nonce: u64,
    pub extrinsic_hash: String,
    pub success: bool,
    pub params: Vec<KusamaTxStatusResponseDataParam>,
    // Option of something, I'm leaving it like that until it blows up
    pub transfer: Option<String>,
    pub event: Vec<KusamaTxStatusResponseDataEvent>,
    pub event_count: u64,
    pub fee: String,
    pub fee_used: String,
    pub error: KusamaTxStatusResponseDataError,
    pub finalized: bool,
    pub lifetime: KusamaTxStatusResponseDataLifetime,
    pub tip: u64,
    pub account_display: KusamaTxStatusResponseDataAccountDisplay,
    pub block_hash: String,
    pub pending: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaTxStatusResponse {
    pub code: u64,
    pub message: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub data: KusamaTxStatusResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KusamaTxDecodingResponse {
    // Address
    pub address: String,
    pub asset_id: u64,
    pub block_hash: String,
    pub block_number: u64,
    pub era: String,
    pub genesis_hash: String,
    pub metadata_rpc: String,
    pub method: String,
    pub nonce: String,
    pub signed_extensions: Vec<String>,
    pub spec_version: String,
    pub tip: String,
    pub transaction_version: String,
    pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct KusamaPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signature: String,
}

#[derive(Clone, Debug)]
pub struct KilnKusamaClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnKusamaClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/ksm", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnKusamaClient {
    pub fn get_stakes(
        &self,
        stakes_request: &KusamaGetStakesRequest,
    ) -> Result<ReturnedData<Vec<KusamaStakesResponse>>, ureq::Error> {
        let KusamaGetStakesRequest {
            addresses,
            accounts,
        } = stakes_request;

        let addresses = addresses.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/stakes?addresses={}&accounts={}",
            self.base_url, addresses, accounts
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<KusamaStakesResponse>>>();

        data
    }

    pub fn get_rewards(
        &self,
        reward_request: &KusamaRewardRequest,
    ) -> Result<ReturnedData<Vec<KusamaReward>>, ureq::Error> {
        let KusamaRewardRequest {
            addresses,
            accounts,
            pool_ids,
            format,
            start_date,
            end_date,
        } = reward_request;

        let addresses = addresses.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let pool_ids = pool_ids.join(",");

        let url = format!(
            "{}/rewards?addresses={}&accounts={}&pool_ids={}&format={}&start_date={}&end_date={}",
            self.base_url,
            addresses,
            accounts,
            pool_ids,
            format.as_ref(),
            start_date,
            end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<KusamaReward>>>();

        data
    }

    pub fn get_operations(
        &self,
        operations_request: &KusamaOperationsRequest,
    ) -> Result<ReturnedData<Vec<KusamaOperation>>, ureq::Error> {
        let KusamaOperationsRequest {
            addresses,
            accounts,
            start_date,
            end_date,
        } = operations_request;

        let addresses = addresses.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = format!(
            "{}/operations?addresses={}&accounts={}&start_date={}&end_date={}",
            self.base_url, addresses, accounts, start_date, end_date
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<KusamaOperation>>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: &KusamaReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let KusamaReportsRequest {
            addresses,
            accounts,
        } = reports_request;

        let addresses = addresses.join(",");

        let accounts = accounts
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url: String = format!(
            "{}/reports?addresses={}&accounts={}",
            self.base_url, addresses, accounts
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

    pub fn get_network_stats(
        &self,
    ) -> Result<ReturnedData<KusamaNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<KusamaNetworkStatsResponse>>();

        data
    }

    pub fn post_bond_tx(
        &self,
        bond_tx_request: &KusamaBondTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_bond_extra_tx(
        &self,
        bond_extra_tx_request: &KusamaBondExtraTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond-extra", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_extra_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_rebond_tx(
        &self,
        rebond_tx_request: &KusamaRebondTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/rebond", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(rebond_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_unbond_tx(
        &self,
        unbond_tx_request: &KusamaUnbondTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/unbond", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unbond_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_nominate_tx(
        &self,
        nominate_tx_request: &KusamaNominateTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/nominate", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(nominate_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_withdraw_unbonded_tx(
        &self,
        withdraw_unbonded_tx_request: &KusamaWithdrawUnbondedTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/withdraw-unbonded", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_unbonded_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_chill_tx(
        &self,
        chill_tx_request: &KusamaChillTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/chill", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(chill_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_set_payee_tx(
        &self,
        join_pool_tx_request: &KusamaSetPayeeTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/set-payee", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(join_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_join_pool_tx(
        &self,
        join_pool_tx_request: &KusamaJoinPoolTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/join-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(join_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_bond_extra_pool_tx(
        &self,
        bond_extra_pool_tx_request: &KusamaBondExtraPoolTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond-extra-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_extra_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_bond_rewards_to_pool_tx(
        &self,
        bond_rewards_to_pool_tx_request: &KusamaPoolTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond-rewards-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_rewards_to_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_claim_payout_from_pool_tx(
        &self,
        claim_tx_request: &KusamaPoolTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/claim-payout-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(claim_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_unbond_from_pool_tx(
        &self,
        unbond_from_pool_tx_request: &KusamaUnbondFromPoolTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/unbond-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unbond_from_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn post_withdraw_unbonded_from_pool_tx(
        &self,
        pool_tx_request: &KusamaPoolTxRequest,
    ) -> Result<ReturnedData<KusamaTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/withdraw-unbonded-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxResponse>>();

        data
    }

    pub fn get_tx_status(
        &self,
        tx_hash: &str,
    ) -> Result<ReturnedData<KusamaTxStatusResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/status?tx_hash={}", self.base_url, tx_hash);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<KusamaTxStatusResponse>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<KusamaTxDecodingResponse, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<KusamaTxDecodingResponse>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &KusamaPrepareTxRequest,
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
}
