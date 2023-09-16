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

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMeshMessage {
    pub(crate) biz_seq_no: String,

    pub(crate) unique_id: String,

    pub(crate) topic: String,

    pub(crate) content: String,

    pub(crate) prop: HashMap<String, String>,

    pub(crate) create_time: u64,
}

impl EventMeshMessage {
    pub fn new(
        biz_seq_no: impl Into<String>,
        unique_id: impl Into<String>,
        topic: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            biz_seq_no: biz_seq_no.into(),
            unique_id: unique_id.into(),
            topic: topic.into(),
            content: content.into(),
            prop: HashMap::with_capacity(16),
            create_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn with_biz_seq_no(&mut self, biz_seq_no: impl Into<String>) -> &Self {
        self.biz_seq_no = biz_seq_no.into();
        self
    }

    pub fn with_unique_id(&mut self, unique_id: impl Into<String>) -> &Self {
        self.unique_id = unique_id.into();
        self
    }

    pub fn with_topic(&mut self, topic: impl Into<String>) -> &Self {
        self.topic = topic.into();
        self
    }

    pub fn with_content(&mut self, content: impl Into<String>) -> &Self {
        self.content = content.into();
        self
    }

    pub fn with_create_time(&mut self, create_time: u64) -> &Self {
        self.create_time = create_time;
        self
    }

    pub fn with_prop(&mut self, prop: &HashMap<String, String>) -> &Self {
        prop.iter().for_each(|(key, value)| {
            self.prop.insert(key.to_string(), value.to_string());
        });
        self
    }
}
