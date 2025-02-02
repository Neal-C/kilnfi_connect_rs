mod errors;
mod prelude;
mod returned_data;
mod sdk;

use crate::prelude::*;
use sdk::{KilnAccountClient, KilnDefiClient, KilnOrganisationClient};
use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct NoUrl;
#[derive(Default, Clone)]
pub struct BaseUrl(String);

#[derive(Default, Clone)]
pub struct NoToken;

#[derive(Default, Clone)]
pub struct Token(String);

#[derive(Default, Clone)]
pub struct NotSealed;

#[derive(Default, Clone)]
pub struct Sealed;

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
        KilnAccountClient::new(self)
    }

    pub fn organisations(&self) -> KilnOrganisationClient {
        KilnOrganisationClient::new(self)
    }

    pub fn defi(&self) -> KilnDefiClient {
        KilnDefiClient::new(self)
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
    pub fn seal(self) -> KilnBuilder<BU, T, Sealed> {
        KilnBuilder {
            base_url: self.base_url,
            api_token: self.api_token,
            marker_seal: PhantomData,
        }
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

    pub fn base_url(self, base_url: impl Into<String>) -> KilnBuilder<BaseUrl, T, NotSealed> {
        KilnBuilder {
            base_url: BaseUrl(base_url.into()),
            api_token: self.api_token,
            marker_seal: PhantomData,
        }
    }
}
