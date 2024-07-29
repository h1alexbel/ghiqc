// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use crate::probe::probe_request::ProbeRequest;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

/// Deep Infra model probe.
pub struct ProbeDeepInfra {
    url: String,
    token: String,
}

impl ProbeDeepInfra {
    /// New deep infra probe.
    pub fn new(url: String, token: String) -> ProbeDeepInfra {
        ProbeDeepInfra { url, token }
    }
}

/// Probe message.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeMessage {
    role: String,
    content: String,
}

impl ProbeMessage {
    /// New probe message.
    pub fn new(role: String, content: String) -> ProbeMessage {
        ProbeMessage { role, content }
    }
}

/// Deep Infra response.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeepInfraResponse {
    choices: Vec<ResponseChoice>,
}

/// Response choice.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseChoice {
    message: ResponseMessage,
}

/// Response messages.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    content: String,
}

/// Request payload to Deep Infra.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeepInfraRequest {
    model: String,
    messages: Vec<ProbeMessage>,
}

impl DeepInfraRequest {
    /// New request payload.
    pub fn new(model: String, messages: Vec<ProbeMessage>) -> DeepInfraRequest {
        DeepInfraRequest { model, messages }
    }
}

impl ProbeRequest for ProbeDeepInfra {
    async fn complete(self, messages: Vec<ProbeMessage>) -> String {
        let request = reqwest::Client::new();
        let payload = DeepInfraRequest::new(
            String::from("Phind/Phind-CodeLlama-34B-v2"),
            messages,
        );
        let response = request
            .post(self.url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .body(
                to_string(&payload)
                    .expect("Cannot stringify request payload to probe"),
            )
            .send()
            .await;
        match response {
            Ok(response) => {
                let json =
                    response.json::<DeepInfraResponse>().await.expect("");
                let vec = json.choices;
                let first = &vec[0];
                let message = &first.message;
                let content = &message.content;
                content.clone()
            }
            Err(err) => panic!("Request to probe failed: {}", err),
        }
    }
}
