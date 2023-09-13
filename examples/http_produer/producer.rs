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
use anyhow::Result;
use eventmesh::config::EventMeshHttpClientConfig;
use eventmesh::http::producer::EventMeshHttpProducer;
use eventmesh::model::eventmesh_message::EventMeshMessage;
use std::collections::HashMap;
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = EventMeshHttpClientConfig::new();
    config
        .env("env")
        .idc("idc")
        .sys("eventmesh")
        .pid(process::id().to_string());
    let http_producer = EventMeshHttpProducer::new(config)?;
    let hash_map = HashMap::with_capacity(10);
    let message = EventMeshMessage::new("1", "1", "2", "2222", hash_map);
    http_producer.publish(&message).await?;
    Ok(())
}
