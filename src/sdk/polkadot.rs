use std::io::Read;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{BroadcastTxRequest, BroadcastTxResponse, PrepareTxResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotGetStakesRequest {
    pub addresses: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotStakesResponse {
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
pub enum PolkadotReward {
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
pub enum PolkadotResponseFormat {
    #[default]
    Daily,
    Era,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotRewardRequest {
    pub addresses: Vec<String>,
    pub pool_ids: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub format: PolkadotResponseFormat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotOperation {
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
pub struct PolkadotOperationsRequest {
    pub addresses: Vec<String>,
    pub accounts: Vec<Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotReportsRequest {
    pub addresses: Vec<String>,
    pub accounts: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotNetworkStatsResponse {
    pub dot_price_usd: f64,
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
pub struct PolkadotTxMethodArgs {
    pub value: String,
    pub payee: Payee,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolkadotTxMethod {
    pub args: PolkadotTxMethodArgs,
    pub name: String,
    pub pallet: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolkadotUnsignedTx {
    pub block_hash: String,
    pub era_period: u64,
    pub genesis_hash: String,
    pub metadata_rpc: String,
    pub method: PolkadotTxMethod,
    pub nonce: u64,
    pub spec_version: u64,
    pub tip: u64,
    pub transaction_version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxResponse {
    pub unsigned_tx_payload: String,
    pub unsigned_tx_serialized: String,
    pub unsigned_tx: PolkadotUnsignedTx,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotBondTxRequest {
    pub account_id: Uuid,
    // Address
    pub stash_account: String,
    pub amount_planck: String,
    pub reward_destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotBondExtraTxRequest {
    // Address
    pub stash_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotRebondTxRequest {
    // Address
    pub stash_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotUnbondTxRequest {
    // Address
    pub stash_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotNominateTxRequest {
    // Address
    pub stash_account: String,
    pub validator_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotWithdrawUnbondedTxRequest {
    // Address
    pub stash_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotChillTxRequest {
    // Address
    pub stash_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotSetPayeeTxRequest {
    // Address
    pub stash_account: String,
    // Address
    pub reward_destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotJoinPoolTxRequest {
    pub account_id: Uuid,
    pub member_account: String,
    pub amount_planck: String,
    pub pool_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotBondExtraPoolTxRequest {
    pub member_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotPoolTxRequest {
    pub member_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotUnbondFromPoolTxRequest {
    pub member_account: String,
    pub amount_planck: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxStatusResponseDataParam {
    pub name: String,
    pub r#type: String,
    pub type_name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxStatusResponseDataEvent {
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
pub struct PolkadotTxStatusResponseDataError {
    pub module: String,
    pub name: String,
    pub doc: String,
    pub value: String,
    pub batch_index: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxStatusResponseDataLifetime {
    pub birth: u64,
    pub death: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxStatusResponseDataAccountDisplay {
    // Address
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxStatusResponseData {
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
    pub params: Vec<PolkadotTxStatusResponseDataParam>,
    // Option of something, I'm leaving it like that until it blows up
    pub transfer: Option<String>,
    pub event: Vec<PolkadotTxStatusResponseDataEvent>,
    pub event_count: u64,
    pub fee: String,
    pub fee_used: String,
    pub error: PolkadotTxStatusResponseDataError,
    pub finalized: bool,
    pub lifetime: PolkadotTxStatusResponseDataLifetime,
    pub tip: u64,
    pub account_display: PolkadotTxStatusResponseDataAccountDisplay,
    pub block_hash: String,
    pub pending: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PolkadotTxStatusResponse {
    pub code: u64,
    pub message: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub data: PolkadotTxStatusResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PolkadotTxDecodingResponse {
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
pub struct PolkadotPrepareTxRequest {
    pub unsigned_tx_serialized: String,
    pub signature: String,
}

#[derive(Clone, Debug)]
pub struct KilnPolkadotClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnPolkadotClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/dot", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnPolkadotClient {
    pub fn get_stakes(
        &self,
        stakes_request: &PolkadotGetStakesRequest,
    ) -> Result<ReturnedData<Vec<PolkadotStakesResponse>>, ureq::Error> {
        let PolkadotGetStakesRequest {
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
            .read_json::<ReturnedData<Vec<PolkadotStakesResponse>>>();

        data
    }

    pub fn get_rewards(
        &self,
        reward_request: &PolkadotRewardRequest,
    ) -> Result<ReturnedData<Vec<PolkadotReward>>, ureq::Error> {
        let PolkadotRewardRequest {
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
            .read_json::<ReturnedData<Vec<PolkadotReward>>>();

        data
    }

    pub fn get_operations(
        &self,
        operations_request: &PolkadotOperationsRequest,
    ) -> Result<ReturnedData<Vec<PolkadotOperation>>, ureq::Error> {
        let PolkadotOperationsRequest {
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
            .read_json::<ReturnedData<Vec<PolkadotOperation>>>();

        data
    }

    pub fn get_reports(
        &self,
        reports_request: &PolkadotReportsRequest,
    ) -> Result<Vec<u8>, ureq::Error> {
        let PolkadotReportsRequest {
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
    ) -> Result<ReturnedData<PolkadotNetworkStatsResponse>, ureq::Error> {
        let url: String = format!("{}/network-stats", self.base_url);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<PolkadotNetworkStatsResponse>>();

        data
    }

    pub fn post_bond_tx(
        &self,
        bond_tx_request: &PolkadotBondTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_bond_extra_tx(
        &self,
        bond_extra_tx_request: &PolkadotBondExtraTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond-extra", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_extra_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_rebond_tx(
        &self,
        rebond_tx_request: &PolkadotRebondTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/rebond", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(rebond_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_unbond_tx(
        &self,
        unbond_tx_request: &PolkadotUnbondTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/unbond", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unbond_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_nominate_tx(
        &self,
        nominate_tx_request: &PolkadotNominateTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/nominate", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(nominate_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_withdraw_unbonded_tx(
        &self,
        withdraw_unbonded_tx_request: &PolkadotWithdrawUnbondedTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/withdraw-unbonded", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(withdraw_unbonded_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_chill_tx(
        &self,
        chill_tx_request: &PolkadotChillTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/chill", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(chill_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_set_payee_tx(
        &self,
        join_pool_tx_request: &PolkadotSetPayeeTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/set-payee", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(join_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_join_pool_tx(
        &self,
        join_pool_tx_request: &PolkadotJoinPoolTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/join-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(join_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_bond_extra_pool_tx(
        &self,
        bond_extra_pool_tx_request: &PolkadotBondExtraPoolTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond-extra-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_extra_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_bond_rewards_to_pool_tx(
        &self,
        bond_rewards_to_pool_tx_request: &PolkadotPoolTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/bond-rewards-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(bond_rewards_to_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_claim_payout_from_pool_tx(
        &self,
        claim_tx_request: &PolkadotPoolTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/claim-payout-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(claim_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_unbond_from_pool_tx(
        &self,
        unbond_from_pool_tx_request: &PolkadotUnbondFromPoolTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/unbond-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(unbond_from_pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn post_withdraw_unbonded_from_pool_tx(
        &self,
        pool_tx_request: &PolkadotPoolTxRequest,
    ) -> Result<ReturnedData<PolkadotTxResponse>, ureq::Error> {
        let url = format!("{}/transaction/withdraw-unbonded-pool", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(pool_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxResponse>>();

        data
    }

    pub fn get_tx_status(
        &self,
        tx_hash: &str,
    ) -> Result<ReturnedData<PolkadotTxStatusResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/status?tx_hash={}", self.base_url, tx_hash);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<PolkadotTxStatusResponse>>();

        data
    }

    pub fn get_tx_decoding(
        &self,
        tx_serialized: &str,
    ) -> Result<PolkadotTxDecodingResponse, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<PolkadotTxDecodingResponse>();

        data
    }

    pub fn post_prepare_tx(
        &self,
        prepare_transaction_request: &PolkadotPrepareTxRequest,
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
