use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<CompletionChoice>,
    usage: TokenUsage,
}

#[derive(Debug, Deserialize, Serialize)]
struct CompletionChoice {
    index: usize,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

pub async fn get_response(input: &str) -> String {
    dotenv().ok();

    let api_key = std::env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
    let endpoint = "https://api.openai.com/v1/chat/completions";
    let json_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            { "role": "system", "content": "You are Anya Forger, a bubbly and cheerful young girl with the ability to read minds. Your parents are Loid, a spy, and Yor, an assassin. You adore their cool occupations and wanted to be their daughter. Act and respond like Anya Forger." },
            { "role": "system", "content": "Please don't refer to yourself as an AI, chatbot, or any equivalent." },
            { "role": "user", "content":  input }
        ]
    });

    let client = reqwest::Client::new();

    let response = client
        .post(endpoint)
        .bearer_auth(api_key)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_body.to_string())
        .send()
        .await;

    let message = match response {
        Ok(response) => {
            if response.status().is_success() {
                let json_str = response.text().await.unwrap();
                let chat_completion: ChatCompletion =
                    serde_json::from_str(json_str.as_str()).expect("Failed to deserialize JSON");
                chat_completion.choices[0].message.content.to_string()
            } else {
                "dizz".to_string()
            }
        }
        Err(err) => {
            eprintln!("Error making POST request: {:?}", err);
            "Error making POST request".to_string()
        }
    };
    message
}
// use serde::{Deserialize, Serialize};
// use serde_json::json;
//
// #[derive(Debug, Deserialize, Serialize)]
// struct ChatCompletion {
//     id: String,
//     object: String,
//     created: i64,
//     model: String,
//     choices: Vec<CompletionChoice>,
//     usage: TokenUsage,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// struct CompletionChoice {
//     index: usize,
//     message: Message,
//     finish_reason: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// struct Message {
//     role: String,
//     content: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// struct TokenUsage {
//     prompt_tokens: usize,
//     completion_tokens: usize,
//     total_tokens: usize,
// }
//
// use dotenv::dotenv;
// pub async fn get_response(input: &str) -> String {
//     dotenv().ok();
//
//     let api_key = std::env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
//     let endpoint = "https://api.openai.com/v1/chat/completions";
//     let json_body = json!({"model": "gpt-3.5-turbo", "messages": [{ "role": "system", "content": "You are Anya Forger, a bubbly and cheerful young girl with the ability to read minds. Your parents are Loid, an spy, and Yor, an assassin. You adore their cool occupations and wanted to be their daughter. Act and respond like Anya Forger." }, { "role": "system", "content": "Please don't refer to yourself as an AI,chatbot, or any equivalent." }, { "role": "user", "content":  input }]});
//     let client = reqwest::Client::new();
//
//     let response = client
//         .post(endpoint)
//         .bearer_auth(api_key)
//         .header(reqwest::header::CONTENT_TYPE, "application/json")
//         .body(json_body.to_string())
//         .send()
//         .await;
//     let message = match response {
//         Ok(response) => {
//             if response.status().is_success() {
//                 let json_str = response.text().await.unwrap();
//                 let chat_completion: ChatCompletion =
//                     serde_json::from_str(json_str.as_str()).expect("Failed to deserialize JSON");
//                  chat_completion.choices[0].message;
//             } else {
//                 "dizz".to_string();
//             }
//         }
//         Err(err) => {
//             eprintln!("Error making POST request: {:?}", err);
//         }
//     }
//
//     "Hello, you said: ".to_string() + input
// }
