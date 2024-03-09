use core::panic;
use std::collections::HashMap;

use reqwest::{header::AUTHORIZATION, Client, Method, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use anyhow::Result;

// use crate::types::Error;

const BASE_URL: &'static str = "https://api.spacetraders.io/v2";

pub async fn perform_api_request<T>(request_data: ApiRequestData) -> Result<T>
where
    T: DeserializeOwned,
{
    let client = Client::new();

    let request = build_request(&client, request_data);

    let response = request.send().await?;
    // match response.status() {
    //     StatusCode::OK => {
    //         return Ok(response.json::<T>().await?);
    //     }
    //     other => {
    //         println!("Error: {}", response.text().await?);
    //         panic!("Unhandled http status {:?}", other);
    //     }
    // }

    let status = response.status().as_u16();

    let text = response.text().await?;
    println!("Status: {}", status);
    println!("Answer: {}", &text);

    Ok(serde_json::from_str(&text)?)

    // Ok(request.send().await?.json::<T>().await?)
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

    pub fn content<T: Serialize + Send + 'static>(mut self, content: T) -> ApiRequestData {
        self.additional_configs
            .push(Box::new(JsonContentApiConfig::<T> { content }));

        self
    }

    pub fn authorization(mut self, token: &str) -> ApiRequestData {
        self.additional_configs
            .push(Box::new(AuthorizationApiConfig {
                token: token.to_string(),
            }));

        self
    }

    pub fn paged(mut self, limit: usize, page: usize) -> ApiRequestData {
        self.additional_configs
            .push(Box::new(QueryParameterApiConfig {
                parameters: HashMap::from([
                    ("limit".to_owned(), limit.to_string()),
                    ("page".to_owned(), page.to_string()),
                ]),
            }));

        self
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

struct AuthorizationApiConfig {
    token: String,
}

impl ApiAdditionalConfig for AuthorizationApiConfig {
    fn apply_config(&self, request: RequestBuilder) -> RequestBuilder {
        request.bearer_auth(&self.token)
    }
}

#[derive(Deserialize)]
pub struct ApiData<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct PagedApiData<T> {
    pub data: Vec<T>,
    pub meta: PageMetaData,
}

#[derive(Deserialize)]
pub struct PageMetaData {
    pub total: usize,
    pub page: usize,
    pub limit: usize,
}
