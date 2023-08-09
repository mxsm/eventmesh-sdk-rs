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
use reqwest::Client;
use crate::http::eventmesh_http_config::HttpConfig;
use reqwest::header::{self, HeaderValue};

pub struct EventMeshMessageHttpProuder {
    http_client: Client,
    url: String,
}

impl EventMeshMessageHttpProuder{
    pub fn new(http_config: &HttpConfig) -> anyhow::Result<Self>{
        let headers = header::HeaderMap::with_capacity(10);
        let client = reqwest::ClientBuilder::new().default_headers(headers);
        anyhow::Ok(EventMeshMessageHttpProuder{
            http_client: client.build()?,
            url: "".to_string()
        })
    }
}
