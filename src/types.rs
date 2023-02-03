use std::time::SystemTimeError;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub id: String,
    pub created: usize,
    pub api_key: String,
    pub model_key: String,
    pub model_inputs: Value,
    pub start_only: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckPayload {
    pub id: String,
    pub created: usize,
    pub long_poll: bool,
    pub call_i_d: String,
    pub api_key: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BananaResponse {
    pub id: String,
    pub message: String,
    pub created: usize,
    pub api_version: String,
    pub call_i_d: Option<String>,
    pub finished: Option<bool>,
    pub model_outputs: Option<serde_json::Value>,
}

#[derive(Error, Debug)]
pub enum BananaError {
    #[error("error in parsing json")]
    JsonError(reqwest::Error),

    #[error("server did not return 200")]
    ServerError(String),

    #[error("Connection to server could not be established")]
    ConnectionError(reqwest::Error),

    #[error("Error in message from server")]
    ModelError(String),

    #[error("Error in response from Banana server")]
    ResponseError(String),

    #[error("Faild to get system time")]
    TimeError(SystemTimeError)
}