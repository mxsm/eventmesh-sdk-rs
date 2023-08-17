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
use crate::http::producer_client::Producer;
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
pub struct EventMeshResponse {
    pub res_time: i64,
    pub ret_code: i32,
    pub ret_msg: String,
}

macro_rules! http_header {
    ($header:expr,$key:expr, $value:expr) => {
        $header.insert($key, HeaderValue::from_str(&$value)?);
    };
}

impl EventMeshMessageHttpProuder {
    pub fn new(http_config: &HttpConfig) -> anyhow::Result<Self> {
        let mut http_headers = header::HeaderMap::with_capacity(12);
        http_header!(http_headers,ENV,http_config.env);
        http_header!(http_headers,IDC,http_config.idc);
        http_header!(http_headers,IP,Ipv4Addr::LOCALHOST.to_string());
        http_header!(http_headers,PID,std::process::id().to_string());
        http_header!(http_headers,SYS,http_config.sys);
        http_header!(http_headers,USERNAME,http_config.user_name);
        http_header!(http_headers,PASSWD,http_config.password);
        http_header!(http_headers,VERSION,protocol_version::ProtocolVersion::V1.to_string());
        http_header!(http_headers,PROTOCOL_TYPE,EM_MESSAGE_PROTOCOL);
        http_header!(http_headers,PROTOCOL_DESC,"http");
        http_header!(http_headers,PROTOCOL_VERSION,"1.0");
        http_header!(http_headers,LANGUAGE,"Rust");
        let client = reqwest::ClientBuilder::new().default_headers(http_headers);
        anyhow::Ok(EventMeshMessageHttpProuder {
            http_client: client.build()?,
            url: http_config.lite_eventmesh_addr.clone(),
        })
    }
}

impl Producer<EventMeshMessage> for EventMeshMessageHttpProuder  {

    async fn publish(&self,msg: &EventMeshMessage) {
            let response = self.http_client
                .post(&self.url)
                .header("code","104")
                .json(msg).send().await?;
            let rsp: EventMeshResponse = response.json().await?;
            println!("{:?}", rsp);
        let result = anyhow::Ok(rsp).unwrap();
    }

    async fn request(&self,msg: &EventMeshMessage, time_out: u32) -> anyhow::Result<EventMeshMessage> {
        todo!()
    }

    async fn request_callback(&self,msg: &EventMeshMessage, time_out: u32, rr_call_back: &dyn Fn(anyhow::Result<EventMeshMessage>)) -> anyhow::Result<()> {
        todo!()
    }


}

