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

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use thiserror;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum ErrorType {
    #[error("Http request error")]
    HttpRequestError,

    #[error("Json parse error")]
    JsonParseError,

    #[error("Event mesh handle error, code:{0}")]
    EventMeshRespError(i32),
}

#[derive(Debug)]
pub struct EventMeshError {
    pub(crate) error_type: Option<ErrorType>,
    pub(crate) message: Option<String>,
    pub(crate) source: Option<anyhow::Error>,
}

impl Default for EventMeshError {
    fn default() -> Self {
        Self {
            error_type: None,
            message: None,
            source: None,
        }
    }
}

impl EventMeshError {}

impl Error for EventMeshError {}

impl Display for EventMeshError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(err_type) = &self.error_type {
            write!(f, "{}", err_type)?;
        }

        if let Some(message) = &self.message {
            write!(f, " message = {}", message)?;
        }

        if let Some(source) = &self.source {
            write!(f, " source = {source}")?;
        }
        Ok(())
    }
}
