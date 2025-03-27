use serde::{Deserialize, Serialize};

use crate::{response_format::ReturnedData, Kiln};

use super::{
    BroadcastTxRequest, BroadcastTxResponse, GetBalanceRequest, PrepareTxRequest,
    PrepareTxResponse, TxDecodingResponse, TxResponse, TxStakeCoin, TxStatusResponse,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BurnTxRequest {
    pub pubkey: String,
    pub recipient: String,
    pub amount_uusdc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OsmoIBCTransferTxRequest {
    pub pubkey: String,
    pub recipient: String,
    pub amount_uusdc: String,
}

#[derive(Clone, Debug)]
pub struct KilnNobleClient {
    pub bearer_token: String,
    pub base_url: String,
}

impl From<&Kiln> for KilnNobleClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/noble", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnNobleClient {
    pub fn post_get_balance(
        &self,
        get_balance_request: &GetBalanceRequest,
    ) -> Result<ReturnedData<TxStakeCoin>, ureq::Error> {
        let url: String = format!("{}/balance", self.base_url);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(get_balance_request)?
            .body_mut()
            .read_json::<ReturnedData<TxStakeCoin>>();

        data
    }

    pub fn post_burn_usdc_tx(
        &self,
        burn_tx_request: &BurnTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url = format!("{}/transaction/burn-usdc", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(burn_tx_request)?
            .body_mut()
            .read_json::<ReturnedData<TxResponse>>();

        data
    }

    pub fn post_osmo_ibc_transfer_tx(
        &self,
        osmo_ibc_transfer_tx_request: &OsmoIBCTransferTxRequest,
    ) -> Result<ReturnedData<TxResponse>, ureq::Error> {
        let url = format!("{}/transaction/osmo-ibc-transfer", self.base_url,);

        let data = ureq::post(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .send_json(osmo_ibc_transfer_tx_request)?
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

    pub fn get_tx_decoding(&self, tx_serialized: &str) -> Result<TxDecodingResponse, ureq::Error> {
        let url: String = format!(
            "{}/transaction/decode?tx_serialized={}",
            self.base_url, tx_serialized
        );

        let data = ureq::get(url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<TxDecodingResponse>();

        data
    }
}
