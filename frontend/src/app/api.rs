use serde::{de::DeserializeOwned, Serialize};
use vallheru::api::{ApiError, ApiRequest, ErrorResponse};

static API_URL: &str = "http://nebula-dev.local.mainnet.community:3004";

pub struct ApiRequestBuilder<'a, Q, H, H2, FHE, FUE> {
    query: Q,
    client: Option<&'a reqwest::Client>,
    on_begin_func: Option<H>,
    on_finish_func: Option<H2>,
    on_handled_error_func: Option<FHE>,
    on_unexpected_error_func: Option<FUE>,
}

impl<'a, Q, H, H2, FHE, FUE> ApiRequestBuilder<'a, Q, H, H2, FHE, FUE>
where
    Q: ApiRequest + Serialize,
    H: FnOnce(),
    H2: FnOnce(),
    FHE: FnOnce(ErrorResponse),
    FUE: FnOnce(ApiError),
{
    pub fn new(query: Q) -> Self {
        ApiRequestBuilder {
            query,
            client: None,
            on_begin_func: None,
            on_finish_func: None,
            on_handled_error_func: None,
            on_unexpected_error_func: None,
        }
    }

    pub fn with_client(mut self, client: &'a reqwest::Client) -> Self {
        self.client = Some(client);

        self
    }

    pub fn on_begin(mut self, func: H) -> Self {
        self.on_begin_func = Some(func);

        self
    }

    pub fn on_finish(mut self, func: H2) -> Self {
        self.on_finish_func = Some(func);

        self
    }

    pub fn on_handled_error(mut self, func: FHE) -> Self {
        self.on_handled_error_func = Some(func);

        self
    }

    pub fn on_unexpected_error(mut self, func: FUE) -> Self {
        self.on_unexpected_error_func = Some(func);

        self
    }

    pub async fn send<R>(self, on_successful: impl FnOnce(R))
    where
        R: DeserializeOwned,
    {
        if let Some(handler) = self.on_begin_func {
            handler();
        }

        let req: vallheru::api::Result<R> =
            vallheru::api::api_request(self.client, API_URL, &self.query).await;

        match req {
            Err(err) => match err {
                ApiError::Api(err_response) => {
                    if let Some(handler) = self.on_handled_error_func {
                        handler(err_response);
                    }
                }
                _ => {
                    if let Some(handler) = self.on_unexpected_error_func {
                        handler(err);
                    }
                }
            },
            Ok(res) => on_successful(res),
        }

        if let Some(handler) = self.on_finish_func {
            handler();
        }
    }
}
