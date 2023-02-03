# Banana Rust SDK

### Getting Started

Add via cargo `cargo add banana-rust-sdk`

Get your API Key
-   [Sign in / log in here](https://app.banana.dev/)

### Simple example

Both examples are based on calling a model link in [this template](https://github.com/bananaml/serverless-template).

Note that since models depends on your model, the Banana SDK cannot type check that you model input is correct. The `banana_rust_sdk::run()` takes any valid json (`serde_json::value`) as model input. Below is a more elaborate example with type checking.

```rust
use banana_rust_sdk;
use serde::Serialize;

#[tokio::main]
async fn main() {
    #[derive(Serialize)]
    struct ModelInputs {
        prompt: String
    }
    
    let api_key = "API_KEY";
    let model_key = "MODEL_KEY";
    let model_inputs = ModelInputs {
        prompt: "try to predict the next [MASK] of this sentence.".to_string()
    };

    let model_inputs = serde_json::to_value(model_inputs).unwrap();

    let res = banana_rust_sdk::run(api_key, model_key, model_inputs).await.unwrap();
    let json = serde_json::to_value(res).unwrap();
    println!("{:?}", json);
}
```

### Example with type checking on the input
```rust
use banana_rust_sdk;
use serde::Serialize;
use serde::Deserialize;
use std::{error::Error, fmt};

#[derive(Debug)]
struct CustomError;

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}


#[derive(Serialize)]
struct ModelInputs {
    prompt: String
}


// Here we define the type of what the model should ouput
#[derive(Serialize, Deserialize, Debug)]
struct ResponseObject {
    score: f64,
    sequence: String,
    token: usize,
    token_str: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelOutputs {
    response_object: Vec<ResponseObject>
}


#[tokio::main]
async fn main() {
    
    let api_key = "API_KEY";
    let model_key = "MODEL_KEY";
    let model_inputs = ModelInputs {
        prompt: "try to predict the next [MASK] of this sentence.".to_string()
    };
    let model_inputs = serde_json::to_value(model_inputs).unwrap();

    let model_ouputs = call_banana(api_key, model_key, model_inputs).await.unwrap();

    // And now we can get e.g. the prediction with the highest score
    let item = &model_ouputs.response_object[0];
    let seq = &item.sequence;

    println!("{:?}", seq);
}


async fn call_banana(api_key: &str, model_key: &str, model_inputs: serde_json::Value) -> Result<ModelOutputs, CustomError> {
    match banana_rust_sdk::run(api_key, model_key, model_inputs).await {
        Ok(res) => {
            match res.model_outputs {
                Some(value) => {
                    let model_output: ModelOutputs = serde_json::from_value(value).unwrap();
                    return Ok(model_output)
                },
                None => return Err(CustomError)
            }
        },
        Err(_) => return Err(CustomError)
    }
}
```