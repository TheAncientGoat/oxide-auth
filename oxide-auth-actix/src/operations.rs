use crate::{OAuthRequest, OAuthResponse, OAuthOperation, WebError};
use oxide_auth::{
    endpoint::{AccessTokenFlow, AuthorizationFlow, Endpoint, RefreshFlow, ResourceFlow},
    primitives::grant::Grant,
};

use actix::prelude::*;
/// Authorization-related operations
#[derive(Message)]
#[rtype(result = "OAuthRequest")]
pub struct Authorize(pub OAuthRequest);

impl OAuthOperation for Authorize {
    type Item = OAuthResponse;
    type Error = WebError;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        AuthorizationFlow::prepare(endpoint)?
            .execute(self.0)
            .map_err(WebError::from)
    }
}

/// Token-related operations
#[derive(Message)]
#[rtype(result = "OAuthRequest")]
pub struct Token(pub OAuthRequest);

impl OAuthOperation for Token {
    type Item = OAuthResponse;
    type Error = WebError;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        AccessTokenFlow::prepare(endpoint)?
            .execute(self.0)
            .map_err(WebError::from)
    }
}

/// Refresh-related operations
#[derive(Message)]
#[rtype(result = "OAuthRequest")]
pub struct Refresh(pub OAuthRequest);

impl OAuthOperation for Refresh {
    type Item = OAuthResponse;
    type Error = WebError;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        RefreshFlow::prepare(endpoint)?
            .execute(self.0)
            .map_err(WebError::from)
    }
}

/// Resource-related operations
#[derive(Message)]
#[rtype(result = "OAuthRequest")]
pub struct Resource(pub OAuthRequest);

impl OAuthOperation for Resource {
    type Item = Grant;
    type Error = Result<OAuthResponse, WebError>;

    fn run<E>(self, endpoint: E) -> Result<Self::Item, Self::Error>
    where
        E: Endpoint<OAuthRequest>,
        WebError: From<E::Error>,
    {
        ResourceFlow::prepare(endpoint)
            .map_err(|e| Err(WebError::from(e)))?
            .execute(self.0)
            .map_err(|r| r.map_err(WebError::from))
    }
}
