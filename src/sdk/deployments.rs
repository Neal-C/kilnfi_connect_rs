use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use uuid::Uuid;

use crate::{response_format::ReturnedData, Kiln};

use super::{Chain, ChainID, StakeStatus};

#[derive(Serialize, Deserialize, EnumString, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProductType {
    Defi,
    Dedicated,
    Pooling,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Deployment {
    pub id: Uuid,
    pub product_type: DeploymentProductType,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub chain: Chain,
    pub chain_id: ChainID,
    pub address: String,
    pub status: StakeStatus,
    pub asset_icon: Option<String>,
    pub protocol_icon: Option<String>,
    pub product_fee: String,
}

#[derive(Clone, Debug)]
pub struct KilnDeploymentsClient {
    bearer_token: String,
    base_url: String,
}

impl From<&Kiln> for KilnDeploymentsClient {
    fn from(kiln: &Kiln) -> Self {
        let url: String = format!("{}/deployments", kiln.base_url);
        let bearer_token: String = format!("Bearer {}", kiln.api_token);

        Self {
            bearer_token,
            base_url: url,
        }
    }
}

impl KilnDeploymentsClient {
    pub fn get(&self) -> Result<ReturnedData<Vec<Deployment>>, ureq::Error> {
        let data = ureq::get(&self.base_url)
            .header("accept", "application/json; charset=utf-8")
            .header("Authorization", &self.bearer_token)
            .call()?
            .body_mut()
            .read_json::<ReturnedData<Vec<Deployment>>>();

        data
    }
}
