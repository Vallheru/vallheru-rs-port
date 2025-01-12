use super::player_state::ApiToken;
use leptos::prelude::*;
use reqwest::Client as ReqClient;
use serde::{de::DeserializeOwned, Serialize};
use vallheru::api::{api_request, ApiError, ApiRequest, Result};

#[derive(Clone)]
pub struct Client {
    client: ReqClient,
    token: Option<String>,
    api_url: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: ReqClient::new(),
            token: None,
            api_url: String::from("http://nebula-dev.local.mainnet.community:3004"),
        }
    }

    pub fn new_with_client(client: ReqClient) -> Self {
        Self {
            client,
            token: None,
            api_url: String::from("http://nebula-dev.local.mainnet.community:3004"),
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn with_stored_token<T>(mut self) -> Self
    where
        T: ApiToken + Clone + Send + Sync + 'static,
    {
        let state = use_context::<ReadSignal<T>>()
            .expect("expected player_state::Context to be initialized");

        self.token = Some(state.read().get_token());

        self
    }

    pub async fn send<Q, R>(&self, query: &Q) -> Result<R>
    where
        Q: ApiRequest + Serialize,
        R: DeserializeOwned,
    {
        api_request(Some(&self.client), &self.api_url, query, self.token.clone()).await
    }

    pub async fn send_and_unwrap<Q, R>(
        &self,
        query: &Q,
        on_error: impl FnOnce(&ApiError),
    ) -> Option<R>
    where
        Q: ApiRequest + Serialize,
        R: DeserializeOwned + 'static,
    {
        let result: Result<R> =
            api_request(Some(&self.client), &self.api_url, query, self.token.clone()).await;

        match result {
            Ok(res) => Some(res),
            Err(ref err) => {
                on_error(err);
                None
            }
        }
    }
}
