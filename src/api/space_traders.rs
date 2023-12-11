use std::collections::HashMap;

use reqwest::{Client, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::types::Error;

const BASE_URL: &'static str = "https://api.spacetraders.io/v2";

pub async fn perform_api_request<T>(request_data: ApiRequestData) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let client = Client::new();

    Ok(build_request(&client, request_data)
        .send()
        .await?
        .json::<T>()
        .await?)
}

fn build_request(client: &Client, request_data: ApiRequestData) -> RequestBuilder {
    let url = format!("{}/{}", BASE_URL, &request_data.endpoint);

    let mut request = client.request(request_data.method, url);

    for config in request_data.additional_configs.iter() {
        request = config.apply_config(request);
    }

    request
}

pub struct ApiRequestData {
    endpoint: String,
    method: Method,
    additional_configs: Vec<Box<dyn ApiAdditionalConfig + Send>>,
}

impl ApiRequestData {
    pub fn new(method: Method, endpoint: &str) -> ApiRequestData {
        ApiRequestData {
            endpoint: endpoint.to_owned(),
            method,
            additional_configs: vec![],
        }
    }

    pub fn new_paged(method: Method, endpoint: &str, limit: usize, page: usize) -> ApiRequestData {
        ApiRequestData {
            endpoint: endpoint.to_owned(),
            method,
            additional_configs: vec![Box::new(QueryParameterApiConfig {
                parameters: HashMap::from([
                    ("limit".to_owned(), limit.to_string()),
                    ("page".to_owned(), page.to_string()),
                ]),
            })],
        }
    }

    pub fn new_content<T: Serialize>(method: Method, endpoint: &str, content: T) -> ApiRequestData
    where
        T: Send + 'static,
    {
        ApiRequestData {
            endpoint: endpoint.to_owned(),
            method,
            additional_configs: vec![Box::new(JsonContentApiConfig::<T> { content })],
        }
    }
}

pub trait ApiAdditionalConfig {
    fn apply_config(&self, request: RequestBuilder) -> RequestBuilder;
}

pub struct JsonContentApiConfig<T: Serialize> {
    content: T,
}

impl<T: Serialize> ApiAdditionalConfig for JsonContentApiConfig<T> {
    fn apply_config(&self, request: RequestBuilder) -> RequestBuilder {
        request.json(&self.content)
    }
}

pub struct QueryParameterApiConfig {
    parameters: HashMap<String, String>,
}

impl ApiAdditionalConfig for QueryParameterApiConfig {
    fn apply_config(&self, request: RequestBuilder) -> RequestBuilder {
        request.query(&self.parameters)
    }
}

#[derive(Deserialize)]
pub struct PagedData<T> {
    pub data: Vec<T>,
    pub meta: PageMetaData,
}

#[derive(Deserialize)]
pub struct PageMetaData {
    pub total: usize,
    pub page: usize,
    pub limit: usize,
}
