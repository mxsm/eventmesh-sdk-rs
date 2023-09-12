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
#![allow(unused)]

use crate::config::EventMeshHttpClientConfig;
use crate::error::ErrorType::{EventMeshRespError, HttpRequestError, JsonParseError};
use crate::error::EventMeshError;
use crate::model::eventmesh_message::{Body, EventMeshMessage, EventMeshResponse};
use crate::protocol_key::{
    ClientInstanceKey, ProtocolConstant, ProtocolKey, ProtocolVersion, RequestCode,
};
use anyhow::{anyhow, Result};
use http::{header, HeaderValue};
use std::time::Duration;

pub struct EventMeshHttpProducer {
    client: reqwest::Client,
    client_config: EventMeshHttpClientConfig,
}

impl EventMeshHttpProducer {
    pub fn new(config: EventMeshHttpClientConfig) -> Result<Self> {
        let mut header_map = header::HeaderMap::with_capacity(16);
        header_map.insert(
            ClientInstanceKey::IDC.key(),
            HeaderValue::from_str(&config.idc)?,
        );
        header_map.insert(
            ClientInstanceKey::IP.key(),
            HeaderValue::from_str(&config.ip)?,
        );
        header_map.insert(
            ClientInstanceKey::PID.key(),
            HeaderValue::from_str(&config.pid)?,
        );
        header_map.insert(
            ClientInstanceKey::SYS.key(),
            HeaderValue::from_str(&config.sys)?,
        );
        header_map.insert(
            ClientInstanceKey::ENV.key(),
            HeaderValue::from_str(&config.env)?,
        );
        header_map.insert(
            ClientInstanceKey::USERNAME.key(),
            HeaderValue::from_str(&config.user_name)?,
        );
        header_map.insert(
            ClientInstanceKey::PASSWD.key(),
            HeaderValue::from_str(&config.password)?,
        );
        header_map.insert(
            ProtocolKey::VERSION,
            HeaderValue::from_str(ProtocolVersion::value(&ProtocolVersion::V1))?,
        );
        header_map.insert(
            ProtocolKey::PROTOCOL_VERSION,
            HeaderValue::from_str(ProtocolVersion::value(&ProtocolVersion::V1))?,
        );
        header_map.insert(
            ProtocolKey::PROTOCOL_TYPE,
            HeaderValue::from_str(ProtocolConstant::EM_MESSAGE_PROTOCOL)?,
        );
        header_map.insert(
            ProtocolKey::PROTOCOL_DESC,
            HeaderValue::from_str(ProtocolConstant::PROTOCOL_DESC)?,
        );
        header_map.insert(ProtocolKey::LANGUAGE, HeaderValue::from_str("Rust")?);
        let client = reqwest::ClientBuilder::new()
            .default_headers(header_map)
            .build()?;

        Ok(EventMeshHttpProducer {
            client,
            client_config: config,
        })
    }
}

impl EventMeshHttpProducer {
    #[cfg(feature = "async")]
    pub async fn publish(&self, message: &EventMeshMessage) -> Result<()> {
        let response = self
            .client
            .post(&self.client_config.lite_event_mesh_addr)
            .timeout(Duration::from_secs(3))
            .header(
                ProtocolKey::REQUEST_CODE,
                RequestCode::get_code(&(RequestCode::MsgSendAsync)).to_string(),
            )
            .form(message)
            .send()
            .await;

        if let Err(err) = response {
            return Err(anyhow!(EventMeshError {
                error_type: Some(HttpRequestError),
                message: Some("Http publish to eventmesh error".to_string()),
                source: Some(err.into())
            }));
        }
        let result = response?.json::<EventMeshResponse>().await;
        if let Err(err) = result {
            return Err(anyhow!(EventMeshError {
                error_type: Some(JsonParseError),
                message: Some("Parse json to EventMeshResponse error".to_string()),
                source: Some(err.into())
            }));
        }
        let em_resp = result?;
        if em_resp.ret_code != 0 {
            return Err(anyhow!(EventMeshError {
                error_type: Some(EventMeshRespError(em_resp.ret_code)),
                message: Some(String::from(em_resp.ret_msg)),
                source: None
            }));
        }
        Ok(())
    }

    pub async fn request(
        &self,
        message: &EventMeshMessage,
        time_out: u32,
    ) -> Result<&EventMeshMessage> {
        todo!()
    }

    pub async fn request_with_callback(
        &self,
        message: &EventMeshMessage,
        time_out: u32,
        call_back: &dyn Fn(Result<EventMeshMessage>),
    ) {
        todo!()
    }
}
