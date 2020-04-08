// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and 
// limitations under the License.

extern crate wascc_actor as actor;
#[macro_use]
extern crate log;
extern crate serde;
extern crate wascc_codec;

use actor::prelude::*;
use serde::Serialize;
use wascc_codec::logging::*;
use wascc_codec::serialize;

actor_handlers! { codec::http::OP_HANDLE_REQUEST => hello_world, 
                  codec::core::OP_HEALTH_REQUEST => health }

fn hello_world(
   r: codec::http::Request) -> CallResult {
    warn!("warn something");
    info!("info something");
    actor::println(&format!("ctx-println Received HTTP request: {:?}", &r));
    let echo = EchoRequest {
        method: r.method,
        path: r.path,
        query_string: r.query_string,
        body: r.body,
    };
    let l = WriteLogRequest {
        level: 3, // should this be a constant??
        body: "raw msg I'm a Body!".to_string(), 
    };

    untyped::default().call("wascc:logging", "WriteLog", wascc_codec::serialize(l)?)?;
    let resp = codec::http::Response::json(echo, 200, "OK");

    logger::default().error("error body").unwrap();
    Ok(serialize(resp)?)
}

fn health(
    _req: codec::core::HealthRequest
) -> ReceiveResult {
    Ok(vec![])
}
#[derive(Serialize)]
struct EchoRequest {
    method: String,
    path: String,
    query_string: String,
    body: Vec<u8>,
}
