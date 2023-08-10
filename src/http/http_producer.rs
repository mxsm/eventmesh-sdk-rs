/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::collections::HashMap;
use std::net::Ipv4Addr;
use reqwest::Client;
use crate::http::eventmesh_http_config::HttpConfig;
use reqwest::header::{self, HeaderValue};
use serde::Deserialize;
use crate::http::protocol_version;
use crate::messages::EventMeshMessage;
use crate::protocol_key::*;

pub struct EventMeshMessageHttpProuder {
    http_client: Client,
    url: String,
}

struct RequestParam {
    query_params: Vec<(String, String)>,
    body: HashMap<String, String>,
    headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Response {
    pub res_time: i64,
    pub ret_code: i32,
    pub ret_msg: String,
}

macro_rules! httpHeader {
    ($header:expr,$key:expr, $value:expr) => {
        $header.insert($key, HeaderValue::from_str(&$value)?);
    };
}

impl EventMeshMessageHttpProuder {
    pub fn new(http_config: &HttpConfig) -> anyhow::Result<Self> {
        let mut http_headers = header::HeaderMap::with_capacity(12);
        httpHeader!(http_headers,ENV,http_config.env);
        httpHeader!(http_headers,IDC,http_config.idc);
        httpHeader!(http_headers,IP,Ipv4Addr::LOCALHOST.to_string());
        httpHeader!(http_headers,PID,std::process::id().to_string());
        httpHeader!(http_headers,SYS,http_config.sys);
        httpHeader!(http_headers,USERNAME,http_config.user_name);
        httpHeader!(http_headers,PASSWD,http_config.password);
        httpHeader!(http_headers,VERSION,protocol_version::ProtocolVersion::V1.to_string());
        httpHeader!(http_headers,PROTOCOL_TYPE,EM_MESSAGE_PROTOCOL);
        httpHeader!(http_headers,PROTOCOL_DESC,"http");
        httpHeader!(http_headers,PROTOCOL_VERSION,"1.0");
        httpHeader!(http_headers,LANGUAGE,"Rust");
        let client = reqwest::ClientBuilder::new().default_headers(http_headers);
        anyhow::Ok(EventMeshMessageHttpProuder {
            http_client: client.build()?,
            url: http_config.lite_eventmesh_addr.clone(),
        })
    }

    pub async fn publish(&self, message: EventMeshMessage) -> anyhow::Result<Response> {
        let response = self.http_client
            .post(&self.url)
            .header("code","104")
            .json(&message).send().await?;
        let json: Response = response.json().await?;
        println!("{:?}", json);
        anyhow::Ok(json)
    }
}
