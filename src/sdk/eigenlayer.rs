use serde::{Deserialize, Serialize};

use crate::{response_format::ReturnedData, Kiln};

use super::ChainID;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OperatorMetadata {
    pub name: String,
    pub website: String,
    pub description: String,
    pub logo: String,
    pub twitter: String,
    pub bluesky: String,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OperatorShare {
    // Address
    pub strategy: String,
    // Address
    pub token: String,
    pub shares: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OperatorAutonomousVerifiableService {
    pub address: String,
    pub metadata: OperatorMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EigenlayerOperator {
    // Address
    pub address: String,
    pub metadata: OperatorMetadata,
    pub shares_breakdown: Vec<OperatorShare>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OperatorResponse {
    pub operator: EigenlayerOperator,
    pub avs: Vec<OperatorAutonomousVerifiableService>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Summary {
    // Address
    pub eigenpod: String,
    // Address
    pub owner: String,
    // Address
    pub delegated_to: String,
    pub eigenpod_queuable_restaked_balance: String,
    pub eigenpod_redelegatable_balance: String,
    pub eigenpod_available_balance: String,
    pub eigenpod_pending_balance: String,
    pub eigenpod_withdrawable_balance: String,
    pub beaconchain_restaked_balance: String,
    pub beaconchain_pending_balance: String,
    pub beaconchain_restakable_balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AutonomousVerifiableServiceReward {
    pub token_address: String,
    pub claimable_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RestakedPoints {
    // Address
    pub validator_address: String,
    pub points: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiquidPoints {
    // Address
    pub strategy: String,
    // Address
    pub token: String,
    pub points: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NativePoints {
    pub total: f64,
    pub breakdown: Vec<RestakedPoints>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiquidPointsResponse {
    pub total: f64,
    pub breakdown: Vec<LiquidPoints>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Undelegation {
    // Address
    pub id: String,
    pub shares: String,
    // Address
    pub staker: String,
    // Addreess
    pub delegated_to: String,
    // Address
    pub withdrawer: String,
    pub nonce: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub tx_hash: String,
    pub claimable: bool,
    pub claimable_at_block: u64,
    pub claimed: bool,
    pub claimed_tx_hash: String,
    pub withdrawable_as_eth: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiquidDeposit {
    // Address
    pub id: String,
    // Address
    pub staker: String,
    // Address
    pub strategy: String,
    // Address
    pub token: String,
    pub shares: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EigenlayerToken {
    // Address
    pub strategy: String,
    // Address
    pub token: String,
    pub shares: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EigenlayerWithdrawal {
    // hash
    pub id: String,
    // Address
    pub withdrawer: String,
    // Address
    pub staker: String,
    // Address
    pub delegated_to: String,
    pub nonce: u64,
    pub tokens: Vec<EigenlayerToken>,
    pub block_number: u64,
    pub timestamp: u64,
    pub tx_hash: String,
    pub claimable: bool,
    pub claimable_at_block: u64,
    pub claimed: bool,
    pub claimed_tx_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EigenlayerTxRequest {
    // Address
    pub earner_address: String,
    // Address
    pub receiver_address: String,
    pub token_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EigenlayerTxResponse {
    // hash
    pub unsigned_tx_hash: String,
    pub unsigned_tx_serialized: String,
    // Address
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
pub struct EigenlayerPostTx {
    // Address
    pub wallet: String,
    pub validator_indexes: Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct KilnEigenlayerClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnEigenlayerClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/eth/eigenlayer", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnEigenlayerClient {
    pub fn eigenpod(&self, wallet: &str) -> Result<ReturnedData<String>, ureq::Error> {
        let url: String = format!("{}/eigenpod?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<String>>();

        data
    }

    pub fn operator(
        &self,
        operator_address: &str,
    ) -> Result<ReturnedData<OperatorResponse>, ureq::Error> {
        let url: String = format!("{}/operator?operator={}", self.base_url, operator_address);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<OperatorResponse>>();

        data
    }

    pub fn summary(&self, wallet: &str) -> Result<ReturnedData<Summary>, ureq::Error> {
        let url: String = format!("{}/summary?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Summary>>();

        data
    }

    pub fn autonomous_verifiable_service_rewards(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<Vec<AutonomousVerifiableServiceReward>>, ureq::Error> {
        let url: String = format!("{}/avs-rewards?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<AutonomousVerifiableServiceReward>>>();

        data
    }

    pub fn avs_rewards(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<Vec<AutonomousVerifiableServiceReward>>, ureq::Error> {
        self.autonomous_verifiable_service_rewards(wallet)
    }

    pub fn native_points(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<Vec<AutonomousVerifiableServiceReward>>, ureq::Error> {
        let url: String = format!("{}/native/points?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<AutonomousVerifiableServiceReward>>>();

        data
    }

    pub fn undelegation(&self, wallet: &str) -> Result<ReturnedData<Undelegation>, ureq::Error> {
        let url: String = format!("{}/native/undelegations?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Undelegation>>();

        data
    }

    pub fn liquid_points(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<LiquidPointsResponse>, ureq::Error> {
        let url: String = format!("{}/liquid/points?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<LiquidPointsResponse>>();

        data
    }

    pub fn liquid_deposits(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<Vec<LiquidDeposit>>, ureq::Error> {
        let url: String = format!("{}/liquid/deposits?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<LiquidDeposit>>>();

        data
    }

    pub fn withdrawals(
        &self,
        wallet: &str,
    ) -> Result<ReturnedData<Vec<EigenlayerWithdrawal>>, ureq::Error> {
        let url: String = format!("{}/liquid/withdrawals?wallet={}", self.base_url, wallet);

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<EigenlayerWithdrawal>>>();

        data
    }

    pub fn claim_rewards_tx(
        &self,
        claim_rewards_tx_request: EigenlayerTxRequest,
    ) -> Result<ReturnedData<EigenlayerTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/claim-rewards", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(claim_rewards_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<EigenlayerTxResponse>>();

        data
    }

    pub fn generate_checkpoint_proofs_tx(
        &self,
        generate_checkpoint_proofs_tx: EigenlayerPostTx,
    ) -> Result<ReturnedData<EigenlayerTxResponse>, ureq::Error> {
        let url: String = format!("{}/transaction/verify-checkpoint-proofs", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(generate_checkpoint_proofs_tx)?
            .body_mut()
            .read_json::<ReturnedData<EigenlayerTxResponse>>();

        data
    }

    pub fn verify_withdraw_credentials_tx(
        &self,
        verify_withdraw_credentials_tx: EigenlayerPostTx,
    ) -> Result<ReturnedData<EigenlayerTxResponse>, ureq::Error> {
        let url: String = format!(
            "{}/transaction/verify-withdrawal-credentials",
            self.base_url
        );

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(verify_withdraw_credentials_tx)?
            .body_mut()
            .read_json::<ReturnedData<EigenlayerTxResponse>>();

        data
    }
}
