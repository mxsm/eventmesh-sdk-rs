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
 use std::time::SystemTime;
 use serde::{Deserialize, Serialize};
 
 #[derive(Debug, Clone,Deserialize,Serialize)]
 pub struct EventMeshMessage {
     #[serde(rename = "bizseqno")]
     pub biz_seq_no: String,
     #[serde(rename = "uniqueid")]
     pub unique_id: String,
     pub topic: String,
     pub content: String,
     pub ttl: String,
     #[serde(rename = "producergroup")]
     pub producer_group:String,
     #[serde(flatten)]
     pub prop: HashMap<String, String>,
     #[serde(rename = "createTime")]
     pub create_time: i64,
 }
 
 impl EventMeshMessage {
     pub fn new(biz_seq_no: &str, unique_id: &str, topic: &str, content: &str, prop: HashMap<String, String>) -> Self {
         Self {
             biz_seq_no: biz_seq_no.to_string(),
             unique_id: unique_id.to_string(),
             topic: topic.to_string(),
             content: content.to_string(),
             ttl:"10".to_string(),
             producer_group: "mxsm".to_string(),
             prop,
             create_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                 .unwrap()
                 .as_millis() as i64,
         }
     }
 }