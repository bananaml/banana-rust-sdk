use std::time::SystemTime;
use reqwest::StatusCode;
use serde_json::Value;
use uuid::Uuid;
use lazy_static::lazy_static;

use crate::types::BananaError;
use crate::types::BananaResponse;
use crate::types::Payload;
use crate::types::CheckPayload;

// getting the url as a global variabel at run time
lazy_static! {
    static ref BANANA_URL: String = {
        match std::env::var("BANANA_URL") {
           Ok(v) => {
            println!("dev mode");
            if v == "local".to_string() {
                return "http://localhost/".to_string();
            } else {
                return v;
            }
        },
            Err(_) => {
                return "https://api.banana.dev/".to_string(); 
            } 
        };
    };
}

pub async fn run_main(api_key: &str, model_key: &str, model_inputs: Value) -> Result<BananaResponse, BananaError> {
    match start_api(api_key, model_key, model_inputs).await {
        Ok(res) => {
            match res.finished {
                Some(value) => {
                    if value == true {
                        return Ok(res)
                    } else {
                        match res.call_i_d {
                            Some(value) => {
                                loop {
                                    println!("polling...");
                                    match check_api(api_key, &value).await {
                                        Ok(res) => {
                                            if res.message.to_ascii_lowercase() == "success" {
                                                return Ok(res)
                                            }
                                        }
                                        Err(e) => return Err(e)
                                    }
                                }         
        
                            },
                            None => return Err(BananaError::ResponseError("call id returned undefined".to_string()))
                        }
                    }
                },
                None => return Err(BananaError::ResponseError("finished returned undefined.".to_string()))
            }
        },
        Err(e) => Err(e)
    }
}

pub async fn start_main(api_key: &str, model_key: &str, model_inputs: Value) -> Result<String, BananaError> {
    match start_api(api_key, model_key, model_inputs).await {
        Ok(res) => {
            match res.call_i_d {
                Some(value) => return Ok(value),
                None => return Err(BananaError::ResponseError("call id returned undefined.".to_string()))
            }
        }  
        Err(e) => Err(e)
    }
}

pub async fn check_main(api_key: &str, call_id: &String) -> Result<BananaResponse, BananaError> {
    check_api(api_key, call_id).await
}

// ------------- API CALLING FUNCTIONS ------------- 
// ------------------------------------------------- 


async fn start_api(api_key: &str, model_key: &str, model_inputs: Value) -> Result<BananaResponse, BananaError> {

    // accessing the global url variable which has to be cloned 
    let mut url_start = BANANA_URL.clone();
    let route_start = "start/v4/";
    url_start.push_str(route_start);
    
    let created = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => time.as_millis() as usize,
        Err(e) => return Err(BananaError::TimeError(e))
    };

    let payload = Payload {
        id: Uuid::new_v4().to_string(),
        created,
        model_key: model_key.to_string(),
        api_key: api_key.to_string(),
        model_inputs,
        start_only: false
    };

    let client = reqwest::Client::new();

    match client.post(&url_start).json(&payload).send().await {
        Ok(res) => {
            
            let status = res.status();
            
            if status != StatusCode::OK {
                Err(BananaError::ServerError(status.to_string()))
            } else {
                match res.json::<BananaResponse>().await {
                    Ok(res) => {
                        if res.message.to_ascii_lowercase().contains("error") {
                            return Err(BananaError::ModelError(res.message))
                        } else {
                            Ok(res)
                        }
                    },
                    Err(e) => Err(BananaError::JsonError(e))

                }    
            }
        },
        Err(e) => Err(BananaError::ConnectionError(e))
    }
}

async fn check_api(api_key: &str, call_id: &String) -> Result<BananaResponse, BananaError> {
    
    let mut url_check = BANANA_URL.clone();
    println!("Hitting endpoint: {}", url_check);
    let route_start = "check/v4/";
    url_check.push_str(route_start);

    let created = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => time.as_millis() as usize,
        Err(e) => return Err(BananaError::TimeError(e))
    };

    let payload = CheckPayload {
        id: Uuid::new_v4().to_string(),
        created,
        long_poll: true,
        call_i_d: call_id.to_string(),
        api_key: api_key.to_string()
    };

    let client = reqwest::Client::new();

    match client.post(url_check).json(&payload).send().await {
        Ok(res) => {
            
            let status = res.status();
            
            if status != StatusCode::OK {
                Err(BananaError::ServerError(status.to_string()))
            } else {
                let json = res.json::<BananaResponse>().await;
                match json {
                    Ok(res) => {
                        if res.message.to_ascii_lowercase().contains("error") {
                            return Err(BananaError::ModelError(res.message))
                        } else {
                            Ok(res)
                        }
                    },
                    Err(e) => Err(BananaError::JsonError(e))
                }    
            }
        },
        Err(e) => Err(BananaError::ConnectionError(e))
    }
}