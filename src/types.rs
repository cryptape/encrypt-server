// Copyright 2018 Cryptape Technology LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_derive::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivatekeyRes {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRawReq {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    pub raw: String,
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureDigestReq {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    pub digest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureRes {
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRawReq {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub signature: String,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationDigestReq {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub signature: String,
    pub digest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRes {
    pub result: bool,
}
