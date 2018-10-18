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

mod router;
mod types;

use std::env;

use actix_web::{http, server, App, middleware};

use self::router::Router;

fn main() {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_e) => "8888".into(),
    };
    let address = format!("0.0.0.0:{}", port);

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("sm2-server");

    server::new(
        || App::new()
            .middleware(middleware::Logger::default())
            .route("/ping", http::Method::GET, Router::ping)
            .route("/sm2/keypair", http::Method::POST, Router::keypair)
            .route("/sm2/raw/signature", http::Method::POST, Router::signature_with_raw)
            .route("/sm2/digest/signature", http::Method::POST, Router::signature_with_digest)
            .route("/sm2/raw/verification", http::Method::POST, Router::verification_with_raw)
            .route("/sm2/digest/verification", http::Method::POST, Router::verification_with_digest)
    ).bind(address.clone())
        .unwrap()
        .start();

    println!("Started http server: {}", address.clone());
    sys.run();
}