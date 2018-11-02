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

use std::fmt;

use actix_web::{HttpRequest, HttpResponse, Json, Path, Responder};
use hex;
use libsm::{
    sm2::signature::{SigCtx, Signature},
    sm3::hash::Sm3Hash,
};

use super::types::{
    PrivatekeyRes, SignatureDigestReq, SignatureRawReq, SignatureRes, VerificationDigestReq,
    VerificationRawReq, VerificationRes,
};

const SIGNATURE_BYTES_LEN: usize = 128;
const PUBKEY_BYTES_LEN: usize = 65;

#[derive(Debug)]
pub enum AppError {
    FromHexError(hex::FromHexError),
    ConverError(bool),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            AppError::FromHexError(ref err) => format!("FromHexError: {}", err),
            AppError::ConverError(_bool) => "ConverError".into(),
        };
        write!(f, "{}", printable)
    }
}

impl From<hex::FromHexError> for AppError {
    fn from(error: hex::FromHexError) -> Self {
        AppError::FromHexError(error)
    }
}

impl From<bool> for AppError {
    fn from(error: bool) -> Self {
        AppError::ConverError(error)
    }
}

pub struct Router {}

impl Router {
    pub fn ping(_info: Path<()>) -> impl Responder {
        format!("pong")
    }

    pub fn keypair(_req: HttpRequest) -> HttpResponse {
        let ctx = SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        let pubk_hex = hex_encode(ctx.serialize_pubkey(&pk, false));
        let privk_hex = hex_encode(ctx.serialize_seckey(&sk));

        HttpResponse::Created().json(PrivatekeyRes {
            public_key: pubk_hex,
            private_key: privk_hex,
        })
    }

    pub fn signature_with_raw(item: Json<SignatureRawReq>) -> HttpResponse {
        let privk_hex = item.private_key.as_str();
        let digest_hex = &into_digest_hex(&item.raw);

        match sign(privk_hex, digest_hex) {
            Ok(signature) => HttpResponse::Created().json(SignatureRes {
                signature: hex_encode(signature),
            }),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }

    pub fn signature_with_digest(item: Json<SignatureDigestReq>) -> HttpResponse {
        let privk_hex = item.private_key.as_str();
        let digest_hex = item.digest.as_str();

        match sign(privk_hex, digest_hex) {
            Ok(signature) => HttpResponse::Created().json(SignatureRes {
                signature: hex_encode(signature),
            }),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }

    pub fn verification_with_raw(item: Json<VerificationRawReq>) -> HttpResponse {
        let pubk_hex = item.public_key.as_str();
        let signature_hex = item.signature.as_str();
        let digest_hex = &into_digest_hex(&item.raw);

        match verify(pubk_hex, signature_hex, digest_hex) {
            Ok(result) => HttpResponse::Ok().json(VerificationRes { result }),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }

    pub fn verification_with_digest(item: Json<VerificationDigestReq>) -> HttpResponse {
        let pubk_hex = item.public_key.as_str();
        let signature_hex = item.signature.as_str();
        let digest_hex = item.digest.as_str();

        match verify(pubk_hex, signature_hex, digest_hex) {
            Ok(result) => HttpResponse::Ok().json(VerificationRes { result }),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}

fn sign(privk_hex: &str, data_hex: &str) -> Result<Vec<u8>, AppError> {
    let privk = hex_decode(privk_hex)?;
    let data = hex_decode(data_hex)?;

    let ctx = SigCtx::new();
    let sk = ctx.load_seckey(&privk)?;
    let pk = ctx.pk_from_sk(&sk);
    let signature = ctx.sign(&data, &sk, &pk);

    let mut sig_bytes = [0u8; SIGNATURE_BYTES_LEN];
    let r_bytes = signature.get_r().to_bytes_be();
    let s_bytes = signature.get_s().to_bytes_be();
    sig_bytes[32 - r_bytes.len()..32].copy_from_slice(&r_bytes[..]);
    sig_bytes[64 - s_bytes.len()..64].copy_from_slice(&s_bytes[..]);
    sig_bytes[64..].copy_from_slice(&ctx.serialize_pubkey(&pk, false)[1..]);
    Ok(sig_bytes.to_vec())
}

fn verify(pubk_hex: &str, signature_hex: &str, data_hex: &str) -> Result<bool, AppError> {
    let pubk = hex_decode(pubk_hex)?;
    let sig_bytes = hex_decode(signature_hex)?;
    if sig_bytes.len() != SIGNATURE_BYTES_LEN {
        return Ok(false);
    }
    let data = hex_decode(data_hex)?;

    if pubk.len() != PUBKEY_BYTES_LEN {
        return Ok(false);
    }
    if pubk[1..] != sig_bytes[64..] {
        return Ok(false);
    }

    let ctx = SigCtx::new();
    let pubk = ctx.load_pubkey(&pubk)?;
    let signature = Signature::new(&sig_bytes[0..32], &sig_bytes[32..64]);
    Ok(ctx.verify(&data, &pubk, &signature))
}

fn into_digest_hex(msg: &str) -> String {
    let data = hex_decode(msg).unwrap();
    let mut hash = Sm3Hash::new(&data);
    let digest = hash.get_hash();
    hex_encode(digest.as_ref())
}

fn hex_decode(msg: &str) -> Result<Vec<u8>, hex::FromHexError> {
    if msg.len() > 2 && msg.starts_with("0x") {
        hex::decode(&msg[2..])
    } else {
        hex::decode(msg)
    }
}

fn hex_encode<T: AsRef<[u8]>>(data: T) -> String {
    format!("0x{}", hex::encode(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_should_eq_signature_len() {
        let ctx = SigCtx::new();
        let (_pk, sk) = ctx.new_keypair();
        let privk_hex = hex_encode(sk.to_bytes_be());
        let data_hex = "0xffff";

        let signature = sign(&privk_hex, data_hex).unwrap();
        assert_eq!(signature.len(), SIGNATURE_BYTES_LEN)
    }

    #[test]
    fn test_verify_should_be_true() {
        let ctx = SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        let pubk_hex = hex_encode(ctx.serialize_pubkey(&pk, false));
        let privk_hex = hex_encode(ctx.serialize_seckey(&sk));
        let data_hex = "0xffff";

        let signature = sign(&privk_hex, data_hex).unwrap();
        let signature_hex = hex_encode(&signature);
        let result = verify(&pubk_hex, &signature_hex, data_hex).unwrap();
        assert_eq!(result, true)
    }

    #[test]
    fn test_verify_should_be_false() {
        let ctx = SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        let pubk_hex = hex_encode(ctx.serialize_pubkey(&pk, false));
        let privk_hex = hex_encode(ctx.serialize_seckey(&sk));
        let data_hex = "0xffff";

        let signature = sign(&privk_hex, data_hex).unwrap();
        let signature_hex = hex_encode(&signature);

        let data_hex = "0xffffff";
        let result = verify(&pubk_hex, &signature_hex, data_hex).unwrap();
        assert_eq!(result, false)
    }
}
