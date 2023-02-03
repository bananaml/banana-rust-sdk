//! The Banana sdk contains three simple asycn functions to call the [Banana](https://www.banana.dev/) services API.
//! 
//! We're moving fast and so we will most likely not prioritize backwards compatibility.
//! The run() function is what you'll use 99% of the time and the other can be seen as helper functions
//! 
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use banana_rust_sdk;
//! use serde::Serialize;
//!
//!#[tokio::main]
//!async fn main() {
//!    #[derive(Serialize)]
//!    struct ModelInputs {
//!        prompt: String
//!    }
//!    
//!    let api_key = "API_KEY";
//!    let model_key = "MODEL_KEY";
//!    let model_inputs = ModelInputs {
//!        prompt: "try to predict the next [MASK] of this sentence.".to_string()
//!    };
//!
//!    let model_inputs = serde_json::to_value(model_inputs).unwrap();
//!
//!    let res = banana_rust_sdk::run(api_key, model_key, model_inputs).await.unwrap();
//!    let json = serde_json::to_value(res).unwrap();
//!    println!("{:?}", json);
//!}
//! ```

use crate::types::BananaError;
use crate::types::BananaResponse;
use crate::utils::run_main;
use crate::utils::check_main;
use crate::utils::start_main;
use serde_json::Value;

pub mod utils;
pub mod types;

/// The main function for calling your model on Banana
/// 
/// # Example
/// ```
/// use banana_rust_sdk;
/// use serde::Serialize;
///
///#[tokio::main]
///async fn main() {
///    #[derive(Serialize)]
///    struct ModelInputs {
///        prompt: String
///    }
///    
///    let api_key = "API_KEY";
///    let model_key = "MODEL_KEY";
///    let model_inputs = ModelInputs {
///        prompt: "try to predict the next [MASK] of this sentence.".to_string()
///    };
///
///    let model_inputs = serde_json::to_value(model_inputs).unwrap();
///
///    let res = banana_rust_sdk::run(api_key, model_key, model_inputs).await.unwrap();
///    let json = serde_json::to_value(res).unwrap();
///    println!("{:?}", json);
///}
/// ```
pub async fn run(api_key: &str, model_key: &str, model_inputs: Value) -> Result<BananaResponse, BananaError> {
    run_main(api_key, model_key, model_inputs).await
}

/// Call API without checking the queue, in general using this should be avoided
pub async fn start(api_key: &str, model_key: &str, model_inputs: Value) -> Result<String, BananaError> {
    start_main(api_key, model_key, model_inputs).await
}

/// Helperfunction to check if there are items in the queue
pub async fn check(api_key: &str, call_id: &String) -> Result<BananaResponse, BananaError> {
    check_main(api_key, call_id).await
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[tokio::test]
    async fn test_run() {
        
        #[derive(Serialize)]
        struct ModelInputs {
            prompt: String
        }
        
        let api_key = "API_KEY";
        let model_key = "MODEL_KEY";
        let model_inputs = ModelInputs {
            prompt: "Paris is the caiptal of [MASK]".to_string()
        };

        let model_inputs = serde_json::to_value(model_inputs).unwrap();

        // Note: since the model inputs are whatevery the user defines them to be, we can't check
        // the types of model inputs in the Banana API
        // Writing a wrapper function around the run function that does #this is recommended.

        run(api_key, model_key, model_inputs).await.unwrap();
    }

    #[tokio::test]
    async fn test_start() {
        #[derive(Serialize)]
        struct ModelInputs {
            prompt: String
        }
        
        let api_key = "API_KEY";
        let model_key = "MODEL_KEY";
        let model_inputs = ModelInputs {
            prompt: "Paris is the capital of [MASK]".to_string()
        };

        let model_inputs = serde_json::to_value(model_inputs).unwrap();

        start(api_key, model_key, model_inputs).await.unwrap();
    }

    #[tokio::test]
    async fn test_check() {
        #[derive(Serialize)]
        struct ModelInputs {
            prompt: String
        }
        
        let api_key = "API_KEY";
        let model_key = "MODEL_KEY";
        let model_inputs = ModelInputs {
            prompt: "Paris is the capital of [MASK]".to_string()
        };

        let model_inputs = serde_json::to_value(model_inputs).unwrap();

        match run(api_key, model_key, model_inputs).await {
            Ok(res) => {
                match check(api_key, &res.call_i_d.unwrap()).await {
                    Ok(out) => println!("{:#?}", out),
                    Err(e) => println!("{:#?}", e)
                };
            },
            Err(_) => {
                panic!("not able to call run() in test_check")
            }
        }
    }
}
