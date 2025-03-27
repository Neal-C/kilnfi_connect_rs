mod errors;
mod prelude;
mod response_format;
mod sdk;

use sdk::{
    account::KilnAccountClient, cardano::KilnCardanoClient, celestia::KilnCelestiaClient,
    cosmos::KilnCosmosClient, defi::KilnDefiClient, deployments::KilnDeploymentsClient,
    dydx::KilnDydxClient, eigenlayer::KilnEigenlayerClient, ethereum::KilnEthereumClient,
    ethereum_onchain::KilnEthereumOnchainClient, fetch_ai::KilnFetchaiClient,
    injective::KilnInjectiveClient, kava::KilnKavaClient, kusama::KilnKusamaClient,
    multiversx::KilnMultiversxClient, near::KilnNearClient, noble::KilnNobleClient,
    organisation::KilnOrganisationClient, osmosis::KilnOsmosisClient, polkadot::KilnPolkadotClient,
    polygon::KilnPolygonClient, solana::KilnSolanaClient, tezos::KilnTezosClient,
    zetachain::KilnZetachainClient,
};

use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct BaseUrl(String);

#[derive(Default, Clone)]
pub struct NoToken;

#[derive(Default, Clone)]
pub struct NoUrl;

#[derive(Default, Clone)]
pub struct NotSealed;

#[derive(Default, Clone)]
pub struct Sealed;

#[derive(Default, Clone)]
pub struct Token(String);

#[derive(Clone, Debug)]
pub struct Kiln {
    api_token: String,
    base_url: String,
}

impl Kiln {
    pub fn builder() -> KilnBuilder<NoUrl, NoToken, NotSealed> {
        KilnBuilder::default()
    }

    pub fn accounts(&self) -> KilnAccountClient {
        KilnAccountClient::from(self)
    }

    pub fn cardano(&self) -> KilnCardanoClient {
        KilnCardanoClient::from(self)
    }

    pub fn celestia(&self) -> KilnCelestiaClient {
        KilnCelestiaClient::from(self)
    }

    pub fn cosmos(&self) -> KilnCosmosClient {
        KilnCosmosClient::from(self)
    }

    pub fn defi(&self) -> KilnDefiClient {
        KilnDefiClient::from(self)
    }

    pub fn deployments(&self) -> KilnDeploymentsClient {
        KilnDeploymentsClient::from(self)
    }

    pub fn dydx(&self) -> KilnDydxClient {
        KilnDydxClient::from(self)
    }

    pub fn eigenlayer(&self) -> KilnEigenlayerClient {
        KilnEigenlayerClient::from(self)
    }

    pub fn ethereum(&self) -> KilnEthereumClient {
        KilnEthereumClient::from(self)
    }

    pub fn ethereum_onchain(&self) -> KilnEthereumOnchainClient {
        KilnEthereumOnchainClient::from(self)
    }

    pub fn fetch_ai(&self) -> KilnFetchaiClient {
        KilnFetchaiClient::from(self)
    }

    pub fn injective(&self) -> KilnInjectiveClient {
        KilnInjectiveClient::from(self)
    }

    pub fn kava(&self) -> KilnKavaClient {
        KilnKavaClient::from(self)
    }

    pub fn kusama(&self) -> KilnKusamaClient {
        KilnKusamaClient::from(self)
    }

    pub fn multiversx(&self) -> KilnMultiversxClient {
        KilnMultiversxClient::from(self)
    }

    pub fn near(&self) -> KilnNearClient {
        KilnNearClient::from(self)
    }

    pub fn noble(&self) -> KilnNobleClient {
        KilnNobleClient::from(self)
    }

    pub fn organisations(&self) -> KilnOrganisationClient {
        KilnOrganisationClient::from(self)
    }

    pub fn osmosis(&self) -> KilnOsmosisClient {
        KilnOsmosisClient::from(self)
    }

    pub fn polkadot(&self) -> KilnPolkadotClient {
        KilnPolkadotClient::from(self)
    }

    pub fn polygon(&self) -> KilnPolygonClient {
        KilnPolygonClient::from(self)
    }

    pub fn solana(&self) -> KilnSolanaClient {
        KilnSolanaClient::from(self)
    }

    pub fn tezos(&self) -> KilnTezosClient {
        KilnTezosClient::from(self)
    }

    pub fn zetachain(&self) -> KilnZetachainClient {
        KilnZetachainClient::from(self)
    }
}

#[derive(Default, Clone, Debug)]
pub struct KilnBuilder<BU, T, Seal> {
    base_url: BU,
    api_token: T,
    marker_seal: PhantomData<Seal>,
}

impl KilnBuilder<NoUrl, NoToken, NotSealed> {
    pub fn new() -> Self {
        KilnBuilder::default()
    }
}

impl<Seal> KilnBuilder<BaseUrl, Token, Seal> {
    pub fn build(self) -> Result<Kiln> {
        Ok(Kiln {
            api_token: self.api_token.0,
            base_url: self.base_url.0,
        })
    }
}

impl<BU, T> KilnBuilder<BU, T, NotSealed> {
    pub fn api_token(self, api_token: impl Into<String>) -> KilnBuilder<BU, Token, NotSealed> {
        KilnBuilder {
            api_token: Token(api_token.into()),
            base_url: self.base_url,
            marker_seal: PhantomData,
        }
    }

    pub fn base_url(self, base_url: &str) -> KilnBuilder<BaseUrl, T, NotSealed> {
        KilnBuilder {
            base_url: BaseUrl(base_url.into()),
            api_token: self.api_token,
            marker_seal: PhantomData,
        }
    }
}

impl<BU, T> KilnBuilder<BU, T, NotSealed> {
    pub fn seal(self) -> KilnBuilder<BU, T, Sealed> {
        KilnBuilder {
            base_url: self.base_url,
            api_token: self.api_token,
            marker_seal: PhantomData,
        }
    }
}
