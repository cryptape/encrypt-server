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

use clap::{Arg, App as CliApp};
use actix_web::{http, server, App, middleware};

use self::router::Router;

fn main() {
    let matches = CliApp::new("sm server")
                          .version("1.0")
                          .author("Jiayu Ye <yejiayu@cryptape.com>")
                          .about("Provider sm algorithm services")
                          .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .help("Set server port. default 8888")
                               .default_value("8888"))
                          .arg(Arg::with_name("ip")
                               .short("i")
                               .long("ip")
                               .help("Set server ip. default 127.0.0.1")
                               .default_value("127.0.0.1"))
                          .get_matches();

    let port = matches.value_of("port").unwrap();
    let ip = matches.value_of("ip").unwrap();
    let address = format!("{}:{}", ip, port);

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
